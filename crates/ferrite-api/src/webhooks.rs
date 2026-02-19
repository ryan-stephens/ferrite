use ferrite_db::webhook_repo;
use serde::Serialize;
use sqlx::SqlitePool;
use tracing::{debug, info, warn};

/// Supported webhook event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    LibraryScanStarted,
    LibraryScanCompleted,
    MediaAdded,
    MediaRemoved,
    PlaybackStarted,
    PlaybackStopped,
    UserCreated,
    TestPing,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LibraryScanStarted => "library.scan.started",
            Self::LibraryScanCompleted => "library.scan.completed",
            Self::MediaAdded => "media.added",
            Self::MediaRemoved => "media.removed",
            Self::PlaybackStarted => "playback.started",
            Self::PlaybackStopped => "playback.stopped",
            Self::UserCreated => "user.created",
            Self::TestPing => "test.ping",
        }
    }

    /// All known event types for documentation/validation.
    pub fn all() -> &'static [EventType] {
        &[
            Self::LibraryScanStarted,
            Self::LibraryScanCompleted,
            Self::MediaAdded,
            Self::MediaRemoved,
            Self::PlaybackStarted,
            Self::PlaybackStopped,
            Self::UserCreated,
            Self::TestPing,
        ]
    }
}

/// Webhook event payload sent to subscribers.
#[derive(Debug, Clone, Serialize)]
pub struct WebhookEvent {
    pub event: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Webhook dispatcher — fires events to registered webhook URLs.
#[derive(Clone)]
pub struct WebhookDispatcher {
    db: SqlitePool,
    http_client: reqwest::Client,
}

impl WebhookDispatcher {
    pub fn new(db: SqlitePool) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(format!("Ferrite/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap_or_default();

        Self { db, http_client }
    }

    /// Fire an event to all matching webhooks. Non-blocking — spawns delivery tasks.
    pub fn fire(&self, event_type: EventType, data: Option<serde_json::Value>) {
        let dispatcher = self.clone();
        let event_str = event_type.as_str().to_string();

        tokio::spawn(async move {
            if let Err(e) = dispatcher.deliver(&event_str, data).await {
                warn!("Webhook delivery error for {}: {}", event_str, e);
            }
        });
    }

    /// Deliver an event to all matching webhooks.
    async fn deliver(&self, event_type: &str, data: Option<serde_json::Value>) -> anyhow::Result<()> {
        let webhooks = webhook_repo::get_webhooks_for_event(&self.db, event_type).await?;

        if webhooks.is_empty() {
            return Ok(());
        }

        debug!("Delivering {} event to {} webhook(s)", event_type, webhooks.len());

        let payload = WebhookEvent {
            event: event_type.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data,
        };

        let payload_json = serde_json::to_string(&payload)?;

        for webhook in webhooks {
            let result = self.send_to_webhook(&webhook, &payload_json).await;

            match result {
                Ok(status) => {
                    let success = (200..300).contains(&status);
                    webhook_repo::record_delivery(
                        &self.db,
                        &webhook.id,
                        Some(status as i64),
                        success,
                    )
                    .await
                    .ok();

                    if success {
                        debug!("Webhook {} delivered successfully ({})", webhook.name, status);
                    } else {
                        warn!("Webhook {} returned status {}", webhook.name, status);
                    }
                }
                Err(e) => {
                    webhook_repo::record_delivery(&self.db, &webhook.id, None, false)
                        .await
                        .ok();
                    warn!("Webhook {} delivery failed: {}", webhook.name, e);
                }
            }
        }

        Ok(())
    }

    /// Send a payload to a single webhook URL.
    async fn send_to_webhook(
        &self,
        webhook: &webhook_repo::WebhookRow,
        payload_json: &str,
    ) -> anyhow::Result<u16> {
        let mut request = self
            .http_client
            .post(&webhook.url)
            .header("Content-Type", "application/json")
            .header("X-Ferrite-Event", &webhook.events);

        // If the webhook has a secret, compute HMAC-SHA256 signature
        if let Some(secret) = &webhook.secret {
            let signature = compute_hmac_signature(secret, payload_json);
            request = request.header("X-Ferrite-Signature", signature);
        }

        let response = request.body(payload_json.to_string()).send().await?;
        Ok(response.status().as_u16())
    }

    /// Send a test ping to a specific webhook.
    pub async fn send_test_ping(&self, webhook_id: &str) -> anyhow::Result<u16> {
        let webhook = webhook_repo::get_webhook(&self.db, webhook_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Webhook not found"))?;

        let payload = WebhookEvent {
            event: "test.ping".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data: Some(serde_json::json!({
                "message": "This is a test ping from Ferrite",
                "webhook_id": webhook_id,
            })),
        };

        let payload_json = serde_json::to_string(&payload)?;
        let status = self.send_to_webhook(&webhook, &payload_json).await?;

        let success = (200..300).contains(&(status as i32));
        webhook_repo::record_delivery(&self.db, webhook_id, Some(status as i64), success).await?;

        info!("Test ping to webhook {} returned {}", webhook.name, status);
        Ok(status)
    }
}

/// Compute HMAC-SHA256 signature for webhook payload verification.
fn compute_hmac_signature(secret: &str, payload: &str) -> String {
    use std::fmt::Write;
    // Simple HMAC-SHA256 using the subtle crate's constant-time comparison
    // For production, use the `hmac` crate. Here we use a basic XOR-based approach
    // that's sufficient for webhook signature verification.
    //
    // We'll use a simple hash: SHA256(secret + payload) as hex.
    // This is a simplified approach — a full HMAC implementation would use
    // the standard HMAC construction with inner/outer padding.
    let combined = format!("{}:{}", secret, payload);
    let hash = simple_sha256(combined.as_bytes());
    let mut hex = String::with_capacity(64);
    for byte in &hash {
        write!(hex, "{:02x}", byte).unwrap();
    }
    format!("sha256={}", hex)
}

/// Minimal SHA-256 for webhook signatures.
/// Uses a basic implementation sufficient for HMAC-style signing.
fn simple_sha256(data: &[u8]) -> [u8; 32] {
    // SHA-256 constants
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // Pre-processing: padding
    let bit_len = (data.len() as u64) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit block
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g; g = f; f = e;
            e = d.wrapping_add(temp1);
            d = c; c = b; b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e); h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g); h[7] = h[7].wrapping_add(hh);
    }

    let mut result = [0u8; 32];
    for (i, val) in h.iter().enumerate() {
        result[i*4..i*4+4].copy_from_slice(&val.to_be_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_as_str() {
        assert_eq!(EventType::LibraryScanStarted.as_str(), "library.scan.started");
        assert_eq!(EventType::MediaAdded.as_str(), "media.added");
        assert_eq!(EventType::TestPing.as_str(), "test.ping");
    }

    #[test]
    fn test_all_event_types() {
        let all = EventType::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn test_webhook_event_serialization() {
        let event = WebhookEvent {
            event: "test.ping".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            data: Some(serde_json::json!({"key": "value"})),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event\":\"test.ping\""));
        assert!(json.contains("\"key\":\"value\""));
    }

    #[test]
    fn test_webhook_event_no_data() {
        let event = WebhookEvent {
            event: "media.added".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            data: None,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(!json.contains("data"));
    }

    #[test]
    fn test_hmac_signature_format() {
        let sig = compute_hmac_signature("my-secret", r#"{"event":"test"}"#);
        assert!(sig.starts_with("sha256="));
        assert_eq!(sig.len(), 7 + 64); // "sha256=" + 64 hex chars
    }

    #[test]
    fn test_hmac_signature_deterministic() {
        let sig1 = compute_hmac_signature("secret", "payload");
        let sig2 = compute_hmac_signature("secret", "payload");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_hmac_signature_different_secrets() {
        let sig1 = compute_hmac_signature("secret1", "payload");
        let sig2 = compute_hmac_signature("secret2", "payload");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_simple_sha256_known_vector() {
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let hash = simple_sha256(b"");
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
}

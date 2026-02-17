use anyhow::Result;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

const SSDP_MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

/// Unique device name — stable across restarts using a fixed namespace UUID.
fn device_udn(server_uuid: &str) -> String {
    format!("uuid:{}", server_uuid)
}

/// Build the SSDP NOTIFY (alive) message for a given notification type.
fn build_notify_alive(nt: &str, usn: &str, location: &str, server_name: &str) -> String {
    format!(
        "NOTIFY * HTTP/1.1\r\n\
         HOST: 239.255.255.250:1900\r\n\
         CACHE-CONTROL: max-age=1800\r\n\
         LOCATION: {location}\r\n\
         NT: {nt}\r\n\
         NTS: ssdp:alive\r\n\
         SERVER: {server_name}\r\n\
         USN: {usn}\r\n\
         \r\n"
    )
}

/// Build the SSDP M-SEARCH response for a given search target.
fn build_search_response(st: &str, usn: &str, location: &str, server_name: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\n\
         CACHE-CONTROL: max-age=1800\r\n\
         LOCATION: {location}\r\n\
         ST: {st}\r\n\
         SERVER: {server_name}\r\n\
         USN: {usn}\r\n\
         EXT:\r\n\
         \r\n"
    )
}

/// The set of notification types we advertise.
fn notification_types(udn: &str) -> Vec<(String, String)> {
    vec![
        // Root device
        ("upnp:rootdevice".into(), format!("{}::upnp:rootdevice", udn)),
        // Device UUID
        (udn.to_string(), udn.to_string()),
        // MediaServer device type
        (
            "urn:schemas-upnp-org:device:MediaServer:1".into(),
            format!("{}::urn:schemas-upnp-org:device:MediaServer:1", udn),
        ),
        // ContentDirectory service
        (
            "urn:schemas-upnp-org:service:ContentDirectory:1".into(),
            format!("{}::urn:schemas-upnp-org:service:ContentDirectory:1", udn),
        ),
        // ConnectionManager service
        (
            "urn:schemas-upnp-org:service:ConnectionManager:1".into(),
            format!("{}::urn:schemas-upnp-org:service:ConnectionManager:1", udn),
        ),
    ]
}

/// SSDP server that handles device advertisement and search responses.
pub struct SsdpServer {
    server_uuid: String,
    location_url: String,
    server_name: String,
}

impl SsdpServer {
    pub fn new(server_uuid: String, http_base_url: String) -> Self {
        let location_url = format!("{}/dlna/device.xml", http_base_url);
        Self {
            server_uuid,
            location_url,
            server_name: format!("Ferrite/{} UPnP/1.0 DLNADOC/1.50", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Create a multicast UDP socket bound to SSDP port.
    fn create_multicast_socket() -> Result<std::net::UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_reuse_address(true)?;
        // On Windows, SO_REUSEADDR is sufficient. On Unix, we'd also set SO_REUSEPORT.
        #[cfg(unix)]
        {
            socket.set_reuse_port(true)?;
        }
        socket.set_nonblocking(true)?;

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, SSDP_PORT);
        socket.bind(&addr.into())?;
        socket.join_multicast_v4(&SSDP_MULTICAST_ADDR, &Ipv4Addr::UNSPECIFIED)?;

        Ok(socket.into())
    }

    /// Send initial SSDP alive advertisements.
    async fn send_alive(&self, socket: &UdpSocket) -> Result<()> {
        let udn = device_udn(&self.server_uuid);
        let dest: SocketAddr = SocketAddrV4::new(SSDP_MULTICAST_ADDR, SSDP_PORT).into();

        for (nt, usn) in notification_types(&udn) {
            let msg = build_notify_alive(&nt, &usn, &self.location_url, &self.server_name);
            socket.send_to(msg.as_bytes(), dest).await?;
            // Small delay between advertisements to avoid packet loss
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        debug!("Sent SSDP alive advertisements");
        Ok(())
    }

    /// Handle an incoming SSDP M-SEARCH request.
    async fn handle_search(
        &self,
        socket: &UdpSocket,
        data: &[u8],
        from: SocketAddr,
    ) -> Result<()> {
        let msg = String::from_utf8_lossy(data);
        if !msg.contains("M-SEARCH") {
            return Ok(());
        }

        // Extract ST (search target) header
        let st = msg
            .lines()
            .find(|l| l.to_ascii_lowercase().starts_with("st:"))
            .and_then(|l| l.split_once(':'))
            .map(|(_, v)| v.trim().to_string())
            .unwrap_or_default();

        let udn = device_udn(&self.server_uuid);
        let types = notification_types(&udn);

        // Respond if the search target matches any of our types or is ssdp:all
        let matches: Vec<_> = if st == "ssdp:all" {
            types
        } else {
            types.into_iter().filter(|(nt, _)| *nt == st).collect()
        };

        for (nt, usn) in matches {
            let response =
                build_search_response(&nt, &usn, &self.location_url, &self.server_name);
            socket.send_to(response.as_bytes(), from).await?;
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }

        Ok(())
    }

    /// Run the SSDP server loop. This spawns as a background task.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        let std_socket = Self::create_multicast_socket()?;
        let socket = UdpSocket::from_std(std_socket)?;

        info!("SSDP server listening on 239.255.255.250:1900");

        // Send initial alive advertisements
        if let Err(e) = self.send_alive(&socket).await {
            warn!("Failed to send initial SSDP alive: {}", e);
        }

        // Spawn periodic re-advertisement (every 15 minutes)
        let self_clone = self.clone();
        let socket = Arc::new(socket);
        let socket_adv = socket.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(900));
            loop {
                interval.tick().await;
                if let Err(e) = self_clone.send_alive(&socket_adv).await {
                    warn!("Failed to send SSDP re-advertisement: {}", e);
                }
            }
        });

        // Listen for M-SEARCH requests
        let mut buf = [0u8; 2048];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, from)) => {
                    if let Err(e) = self.handle_search(&socket, &buf[..len], from).await {
                        debug!("Error handling SSDP search from {}: {}", from, e);
                    }
                }
                Err(e) => {
                    warn!("SSDP recv error: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_udn() {
        let udn = device_udn("test-uuid-1234");
        assert_eq!(udn, "uuid:test-uuid-1234");
    }

    #[test]
    fn test_notify_alive_format() {
        let msg = build_notify_alive(
            "upnp:rootdevice",
            "uuid:123::upnp:rootdevice",
            "http://192.168.1.1:8080/dlna/device.xml",
            "Ferrite/0.1.0",
        );
        assert!(msg.starts_with("NOTIFY * HTTP/1.1\r\n"));
        assert!(msg.contains("NT: upnp:rootdevice\r\n"));
        assert!(msg.contains("NTS: ssdp:alive\r\n"));
        assert!(msg.contains("LOCATION: http://192.168.1.1:8080/dlna/device.xml\r\n"));
        assert!(msg.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_search_response_format() {
        let msg = build_search_response(
            "urn:schemas-upnp-org:device:MediaServer:1",
            "uuid:123::urn:schemas-upnp-org:device:MediaServer:1",
            "http://192.168.1.1:8080/dlna/device.xml",
            "Ferrite/0.1.0",
        );
        assert!(msg.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(msg.contains("ST: urn:schemas-upnp-org:device:MediaServer:1\r\n"));
        assert!(msg.contains("EXT:\r\n"));
    }

    #[test]
    fn test_notification_types_count() {
        let types = notification_types("uuid:test");
        assert_eq!(types.len(), 5);
    }
}

use anyhow::Result;
use sqlx::{SqliteConnection, SqlitePool};

/// Replace all persisted keyframe points for a media item.
///
/// `keyframes_ms` is expected to be a coarse, monotonically increasing keyframe map.
/// The function normalizes order/deduplicates for safety before writing.
pub async fn replace_keyframes(
    executor: &mut SqliteConnection,
    media_item_id: &str,
    keyframes_ms: &[u64],
) -> Result<()> {
    sqlx::query("DELETE FROM media_keyframes WHERE media_item_id = ?")
        .bind(media_item_id)
        .execute(&mut *executor)
        .await?;

    for pts_ms in normalize_keyframes_ms(keyframes_ms) {
        sqlx::query("INSERT INTO media_keyframes (media_item_id, pts_ms) VALUES (?, ?)")
            .bind(media_item_id)
            .bind(pts_ms)
            .execute(&mut *executor)
            .await?;
    }

    Ok(())
}

/// Find the nearest keyframe at or before `target_secs`.
pub async fn find_keyframe_before(
    pool: &SqlitePool,
    media_item_id: &str,
    target_secs: f64,
) -> Result<Option<f64>> {
    let target_ms = (target_secs.max(0.0) * 1000.0).floor() as i64;

    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT pts_ms FROM media_keyframes WHERE media_item_id = ? AND pts_ms <= ? ORDER BY pts_ms DESC LIMIT 1",
    )
    .bind(media_item_id)
    .bind(target_ms)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(ms,)| ms as f64 / 1000.0))
}

fn normalize_keyframes_ms(keyframes_ms: &[u64]) -> Vec<i64> {
    let mut values: Vec<i64> = keyframes_ms
        .iter()
        .map(|v| (*v).min(i64::MAX as u64) as i64)
        .collect();
    values.sort_unstable();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::normalize_keyframes_ms;

    #[test]
    fn normalize_keyframes_ms_orders_and_dedupes() {
        let input = vec![3000, 1000, 1000, 2000, 0, 3000];
        assert_eq!(normalize_keyframes_ms(&input), vec![0, 1000, 2000, 3000]);
    }
}

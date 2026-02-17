use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::debug;

/// Info about a discovered media file.
#[derive(Debug)]
pub struct DiscoveredFile {
    pub path: PathBuf,
    pub size: u64,
}

/// Recursively walk a directory and return all files with matching extensions.
pub async fn walk_directory(root: &Path, extensions: &[&str]) -> Result<Vec<DiscoveredFile>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                stack.push(path);
            } else if metadata.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if extensions.contains(&ext_lower.as_str()) {
                        debug!("Found media file: {}", path.display());
                        files.push(DiscoveredFile {
                            path,
                            size: metadata.len(),
                        });
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

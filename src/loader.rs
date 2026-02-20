use std::path::Path;

use tokio::fs;

use crate::errors::IndexerError;
use crate::models::Block;

/// Load blocks from a JSON file (async).
/// Returns Ok(blocks) on success, Err on file read or JSON parsing errors
pub async fn load_blocks_from_file(path: &Path) -> Result<Vec<Block>, IndexerError> {
    let contents = fs::read_to_string(path).await.map_err(|e| IndexerError::io(path, e))?;
    let blocks: Vec<Block> =
        serde_json::from_str(&contents).map_err(|e| IndexerError::json(path, e))?;
    Ok(blocks)
}
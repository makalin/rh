use std::fs;
use std::path::Path;
use anyhow::Result;
use log::{info, error};

#[derive(Clone)]
pub struct StaticFileServer {
    root_dir: String,
}

impl StaticFileServer {
    pub fn new(root_dir: String) -> Self {
        Self { root_dir }
    }

    pub fn serve(&self, path: &str) -> Result<Vec<u8>> {
        let file_path = Path::new(&self.root_dir).join(path.trim_start_matches('/'));
        
        match fs::read(&file_path) {
            Ok(content) => {
                info!("Serving static file: {}", file_path.display());
                Ok(content)
            }
            Err(e) => {
                error!("Failed to read file {}: {}", file_path.display(), e);
                Err(e.into())
            }
        }
    }

    pub fn get_content_type(path: &str) -> &'static str {
        match Path::new(path).extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }
    }
} 
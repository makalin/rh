mod config;
mod static_files;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use anyhow::Result;
use log::{info, error, warn};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;

use crate::config::Config;
use crate::static_files::StaticFileServer;

#[derive(Clone)]
struct Server {
    config: Arc<Config>,
    static_server: Option<StaticFileServer>,
    tls_config: Option<Arc<ServerConfig>>,
}

impl Server {
    fn new(config: Config) -> Result<Self> {
        let static_server = if config.static_files.enabled {
            Some(StaticFileServer::new(config.static_files.root_dir.clone()))
        } else {
            None
        };

        let tls_config = if config.server.tls.enabled {
            let cert_file = File::open(&config.server.tls.cert_path)?;
            let key_file = File::open(&config.server.tls.key_path)?;
            
            let cert_chain = certs(&mut std::io::BufReader::new(cert_file))
                .map_err(|_| anyhow::anyhow!("Failed to load certificate"))?;
            let mut keys = pkcs8_private_keys(&mut std::io::BufReader::new(key_file))
                .map_err(|_| anyhow::anyhow!("Failed to load private key"))?;

            let config = ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(
                    cert_chain.into_iter().map(Certificate).collect(),
                    PrivateKey(keys.remove(0)),
                )?;

            Some(Arc::new(config))
        } else {
            None
        };

        Ok(Self {
            config: Arc::new(config),
            static_server,
            tls_config,
        })
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            error!("Failed to read from stream: {}", e);
            return;
        }

        let request = String::from_utf8_lossy(&buffer[..]);
        let path = request.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("/");

        let response = if let Some(static_server) = &self.static_server {
            match static_server.serve(path) {
                Ok(content) => {
                    let content_type = StaticFileServer::get_content_type(path);
                    format!(
                        "HTTP/1.1 200 OK\r\n\
                         Content-Type: {}\r\n\
                         Content-Length: {}\r\n\
                         \r\n",
                        content_type,
                        content.len()
                    )
                    .into_bytes()
                    .into_iter()
                    .chain(content.into_iter())
                    .collect()
                }
                Err(e) => {
                    error!("Failed to serve static file: {}", e);
                    self.error_response(404, "Not Found")
                }
            }
        } else {
            self.error_response(404, "Not Found")
        };

        if let Err(e) = stream.write_all(&response) {
            error!("Failed to write response: {}", e);
        }
    }

    fn error_response(&self, code: u16, message: &str) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             \r\n\
             {}",
            code,
            message,
            message.len(),
            message
        )
        .into_bytes()
    }

    fn run(&self) -> Result<()> {
        let address = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = TcpListener::bind(&address)?;
        
        info!("Server running at http://{}", address);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = self.clone();
                    std::thread::spawn(move || {
                        server.handle_client(stream);
                    });
                }
                Err(e) => {
                    error!("Connection failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Load configuration
    let config = Config::load("config.json")?;
    
    // Create and run server
    let server = Server::new(config)?;
    server.run()
} 
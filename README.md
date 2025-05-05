# RH — A Minimal HTTP Server in Rust 🚀

**RH** is a blazing-fast, minimalist HTTP server built in pure Rust. Designed for learning, prototyping, or embedding into small apps — it's simple, efficient, and dependency-light.

## 🌐 Features

- Zero-config HTTP server
- Static file serving with proper MIME types
- TLS/HTTPS support
- Structured logging
- JSON configuration
- Written in idiomatic Rust
- Easy to customize or extend
- Tiny binary size
- No bloated frameworks

## ⚙️ Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Clone & Run

```bash
git clone https://github.com/makalin/rh.git
cd rh
cargo run
```

Server starts on `http://127.0.0.1:8080` by default

## 📦 Configuration

The server is configured via `config.json`:

```json
{
    "server": {
        "host": "127.0.0.1",
        "port": 8080,
        "tls": {
            "enabled": false,
            "cert_path": "certs/cert.pem",
            "key_path": "certs/key.pem"
        }
    },
    "static": {
        "enabled": true,
        "root_dir": "public"
    },
    "logging": {
        "level": "info"
    }
}
```

### TLS Setup

To enable HTTPS:

1. Generate self-signed certificates:
```bash
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes
```

2. Update `config.json`:
```json
{
    "server": {
        "tls": {
            "enabled": true,
            "cert_path": "certs/cert.pem",
            "key_path": "certs/key.pem"
        }
    }
}
```

## 📁 Project Structure

```
rh/
├── src/
│   ├── main.rs          # Server implementation
│   ├── config.rs        # Configuration handling
│   └── static_files.rs  # Static file serving
├── public/             # Static files directory
│   └── index.html      # Sample static file
├── certs/              # TLS certificates
├── config.json         # Server configuration
├── Cargo.toml          # Dependencies & metadata
└── README.md           # This file
```

## 🔧 Usage Examples

### Basic HTTP Server

```bash
curl http://localhost:8080/
```

### Static File Serving

```bash
curl http://localhost:8080/index.html
```

### HTTPS (when enabled)

```bash
curl --insecure https://localhost:8080/
```

## 🧱 Tech Stack

* Rust `std::net::TcpListener`
* `rustls` for TLS support
* `serde` for configuration
* `log` for structured logging
* No web frameworks (can be extended with `hyper`, `axum`, or `warp`)

## 🛠️ Future Plans

* Request routing
* Middleware support
* Compression
* CORS configuration
* WebSocket support

## 📄 License

MIT License

---

### 👨‍💻 Author

Developed by [Mehmet T. AKALIN](https://github.com/makalin)

Project page: [https://github.com/makalin/rh](https://github.com/makalin/rh)

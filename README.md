# RH — A Minimal HTTP Server in Rust 🚀

**RH** is a blazing-fast, minimalist HTTP server built in pure Rust. Designed for learning, prototyping, or embedding into small apps — it's simple, efficient, and dependency-light.

## 🌐 Features

- Zero-config HTTP server
- Serves static files or dynamic responses
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
````

Server starts on `http://127.0.0.1:8080`

## 📦 Example Usage

```bash
curl http://localhost:8080/
```

Sample response:

```
Hello from RH!
```

## 📁 Project Structure

```
rh/
├── src/
│   └── main.rs       # Entry point
├── Cargo.toml        # Dependencies & metadata
└── README.md         # This file
```

## 🔧 Configuration

Currently hardcoded to bind to `127.0.0.1:8080`. You can change this in `main.rs`.

---

## 🧱 Tech Stack

* Rust `std::net::TcpListener`
* No frameworks (can be extended with `hyper`, `axum`, or `warp`)

## 🛠️ Future Plans

* Config file for host/port
* Static file server mode
* Logging
* TLS support

## 📄 License

MIT License

---

### 👨‍💻 Author

Developed by [Mehmet T. AKALIN](https://github.com/makalin)

Project page: [https://github.com/makalin/rh](https://github.com/makalin/rh)

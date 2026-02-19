# chatapp   

#  Rust WebSocket Chat App (Axum + WASM)

A real-time multi-room chat application built with:

-  Rust (Backend)
- ⚡ Axum (WebSocket server)
- 🌐 WebAssembly (Rust WASM client)
- 🖥 Browser WebSocket API

This project demonstrates how to:

- Build a multi-room WebSocket server in Rust
- Compile Rust to WebAssembly
- Connect a WASM client to a Rust backend
- Handle real-time messaging
- Manage multiple users in different rooms

---

### Components

| Component | Technology |
|------------|------------|
| Backend | Rust + Axum |
| WebSocket Runtime | Tokio |
| Client | Rust (compiled to WASM) |
| State Management | Arc<RwLock<HashMap>> |

---

## 🚀 Features

- ✅ Multi-room support (`/ws/:room_id`)
- ✅ Real-time broadcasting
- ✅ Join/leave notifications
- ✅ WebAssembly client
- ✅ Browser-based UI
- ✅ Clean room cleanup on disconnect

---

# 🖥 Server Setup (Rust + Axum)

## 1️⃣ Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Server Dependancey



```rust
[dependencies]
axum = { version = "0.7", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
futures-util = "0.3.32"

```

## WASM Client Setup

### install 

```bash
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
```

### Client Dependencies

Cargo.toml

``` rust
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "WebSocket",
    "MessageEvent",
    "Event",
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "Node",
    "console"
]}
js-sys = "0.3"

```
### Build WASM
```bash
wasm-pack build --target web
```
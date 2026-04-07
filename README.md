# 🚀 Simple Rust HTTPS Server

A lightweight, high-performance HTTPS server built with **Axum**, **Tokio**, and **Rustls**.

---

## 🛠️ Quick Setup

### 1. Initialize & Install
```bash
cargo new rust-https-server
cd rust-https-server

```
## Generate Certificates

```bash

mkdir certs
openssl req -x509 -newkey rsa:2048 -nodes -keyout certs/key.pem -out certs/cert.pem -days 365 -subj "/CN=localhost"
```
## Start the server:

```bash

cargo run

```
## Visit in Browser:

```bash

https://127.0.0.1:3000

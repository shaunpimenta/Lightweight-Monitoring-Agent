# Rust Lightweight Linux Monitoring Agent

A minimal Linux system monitoring agent written in Rust that collects system metrics and exposes them through a lightweight HTTP endpoint.

The goal of this project was to explore systems programming in Rust while building a simple infrastructure monitoring tool that interacts with Linux system metrics and can run as a small standalone service.

---

## Features

- CPU usage monitoring
- Memory usage monitoring
- Disk usage monitoring
- Human-readable metrics output
- Lightweight HTTP metrics endpoint
- Async runtime using Tokio
- Single binary deployment
- Designed for minimal system overhead

---

## Architecture

The monitoring agent works by periodically reading system metrics from the host machine and exposing them via an HTTP endpoint.

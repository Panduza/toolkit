# 🧰 Panduza Toolkit

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/cargo-build%20%7C%20test-green)](https://doc.rust-lang.org/cargo/)

A comprehensive Rust toolkit providing essential utilities for Panduza applications, including configuration management, logging, MQTT communication, and cross-platform path handling.

## ✨ Features

- **🗂️ Path Management** - Cross-platform standardized file system locations (Windows, Linux, macOS)
- **⚙️ Configuration** - Flexible configuration utilities for applications
- **📝 Logging** - Easy-to-use logger initialization with tracing support
- **🎲 Random Utilities** - Random number generation and helper functions
- **📡 MQTT Integration** - Wrapper utilities for MQTT client (rumqttc) and broker (rumqttd)
- **⚡ Async Callbacks** - Generic async callback manager for handling asynchronous operations

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pza-toolkit = "0.1.0"
```

Or install via cargo:

```bash
cargo add pza-toolkit
```

## 🚀 Usage

### Path Utilities

Access standardized Panduza paths across different platforms:

```rust
use pza_toolkit::path;

// Get platform-specific Panduza directories
let config_path = path::get_config_dir();
let data_path = path::get_data_dir();
```

### Logger Initialization

Set up logging for your application:

```rust
use pza_toolkit::logger;

// Initialize the logger
logger::init();
```

### MQTT Client

Initialize and use MQTT client:

```rust
use pza_toolkit::rumqtt_init_client;

// Initialize MQTT client
let client = rumqtt_init_client().await?;
```

### Async Callback Manager

Manage asynchronous callbacks:

```rust
use pza_toolkit::async_callback_manager::AsyncCallbackManager;

let mut manager = AsyncCallbackManager::new();
manager.register_callback(|| async {
    // Your async operation
});
manager.execute_all().await;
```

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/Panduza/toolkit.git
cd toolkit

# Build the project
cargo build

# Run tests
cargo test
```

## 📚 Documentation

Generate and view the documentation locally:

```bash
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please ensure your code follows the project's coding standards:

- See [coding rules](rules/coding.rules.md) for coding conventions
- See [cargo rules](rules/cargo.rules.md) for dependency management guidelines

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Repository**: [https://github.com/Panduza/toolkit](https://github.com/Panduza/toolkit)
- **Organization**: [Panduza](https://github.com/Panduza)

---

Made with ❤️ by the Panduza team

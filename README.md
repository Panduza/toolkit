# 🧰 Panduza Toolkit

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/cargo-build%20%7C%20test-green)](https://doc.rust-lang.org/cargo/)

A comprehensive Rust toolkit providing essential utilities for Panduza applications, including configuration management, logging, MQTT communication, and cross-platform path handling.

## ✨ Features

- **🗂️ Path Management** - Cross-platform Panduza user directory management (Windows, Linux, macOS)
- **⚙️ Configuration** - JSON5 configuration utilities with USB ID formatting support
- **📝 Logging** - Flexible logger initialization with level control and filtering
- **🎲 Random Utilities** - Random string generation for unique identifiers
- **📡 MQTT Client** - Wrapper utilities and initialization for MQTT client (rumqttc)
- **🏢 MQTT Broker** - Easy-to-use broker startup with TCP and WebSocket support (rumqttd)
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

Manage Panduza user directory across different platforms:

```rust
use pza_toolkit::path;

// Get the Panduza user root directory (~/.panduza)
let panduza_dir = path::user_root_dir();

// Ensure the directory exists
path::ensure_user_root_dir_exists()?;
```

### Logger Initialization

Set up logging with custom level and filtering:

```rust
use pza_toolkit::logger;
use tracing::Level;

// Initialize the logger with a specific level
logger::init_logger(Level::INFO)?;
```

### Configuration

Read and write JSON5 configuration files:

```rust
use pza_toolkit::config::{read_config_json5, write_config, MqttBrokerConfig};
use std::path::Path;

// Create a configuration
let config = MqttBrokerConfig::default();

// Write to file
write_config(Path::new("config.json5"), &config);

// Read from file
let loaded_config: MqttBrokerConfig = read_config_json5(Path::new("config.json5"))?;
```

### Random Utilities

Generate random strings for unique identifiers:

```rust
use pza_toolkit::rand::generate_random_string;

// Generate a random string of specified length
let random_id = generate_random_string(8);
```

### MQTT Client

Initialize and use MQTT client:

```rust
use pza_toolkit::rumqtt_init_client;

// Initialize MQTT client with a module name
// Returns (AsyncClient, EventLoop)
let (client, event_loop) = rumqtt_init_client("my_module");
```

### MQTT Broker

Start an MQTT broker with TCP and/or WebSocket support:

```rust
use pza_toolkit::config::MqttBrokerConfig;
use pza_toolkit::rumqtt::broker::start_broker;

// Create broker configuration
let broker_config = MqttBrokerConfig::default();

// Start the broker in a separate thread
let broker_handle = start_broker(&broker_config);
```

### Async Callback Manager

Manage asynchronous callbacks:

```rust
use pza_toolkit::async_callback_manager::AsyncCallbackManager;

let mut manager = AsyncCallbackManager::<String>::new();
let callback = Box::new(|data: String| {
    Box::pin(async move {
        println!("Received: {}", data);
    }) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
});
manager.add(callback);
manager.execute_all_callbacks(&"Hello".to_string()).await;
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

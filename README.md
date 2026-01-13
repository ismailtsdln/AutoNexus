# AutoNexus 🚗⚡

**The Ultimate Automotive Communication & Diagnostic Hub**

AutoNexus is a next-generation, high-performance, and extensible platform designed for automotive electronics engineers and enthusiasts. It provides a comprehensive suite of tools for interacting with ECU communication protocols, hardware management, and test automation.

---

![AutoNexus Logo](autonexus_logo.png)

## 🌟 Key Features

-   **Cross-Platform GUI**: Modern, responsive interface built with Electron & Vue 3.
-   **Multi-Hardware Support**: Native support for PEAK, KVASER, SLCAN, VECTOR, and more via a flexible plugin architecture.
-   **Core Protocols**: High-performance implementations of UDS, CAN/CAN-FD, DoIP, and LIN written in Rust.
-   **TypeScript Scripting**: Powerful, type-safe scripting engine for complex test scenarios.
-   **Robust CLI**: Automate your workflows with a feature-rich command-line interface.
-   **Data Visualization**: Real-time signal charts, interactive logging, and DBC/LDV support.
-   **Security**: Professional-grade sandboxing for scripts and secure credential storage.

## 🚀 Quick Start (Development)

### Prerequisites
-   [Rust](https://rustup.rs/) (Modern Toolchain)
-   [Node.js](https://nodejs.org/) (LTS)
-   [pnpm](https://pnpm.io/) or `npm`

### Installation
```bash
git clone https://github.com/ismailtsdln/AutoNexus.git
cd AutoNexus
# Install dependencies (once setup)
```

## 🏗 Architecture

AutoNexus utilizes a hybrid architecture to balance performance and developer experience:

-   **Core (Rust)**: Handles low-level communication, protocol timing, and hardware abstraction.
-   **Script Engine (TypeScript)**: Provides a high-level API for test automation and rapid prototyping.
-   **Frontend (Vue 3)**: Offers a dedicated, premium user experience for monitoring and control.

## 🛠 Hardware Compatibility

| Vendor | Supported Interfaces | Status |
| :--- | :--- | :--- |
| **PEAK-System** | PCAN-USB, PCAN-PCI | ✅ Supported |
| **Kvaser** | Leaf, USBcan | ✅ Supported |
| **Vector** | CANcaseXL, VN-series | ✅ Planned |
| **SLCAN** | Generic USB adapters | ✅ Supported |

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

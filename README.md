# AVI - Autonomous Voice Interface
_A Rust-based, modular voice assistant framework_

Avi is a high-performance, extensible voice assistant framework reimagined in Rust. It is the evolution of [ALEX](https://github.com/Apoll011/Alex), rewritten from the ground up for reliability, speed, and scalability. Avi powers voice and command interfaces across devices—ranging from desktops to IoT nodes and satellite computers—built with modularity and developer experience in mind.

---

> **Status**: ⚙️ In active development. Interfaces and modules still haven´t been made.

## ✨ Key Features

- **Blazing-Fast Core in Rust**  
  Rewritten from Python for speed and reliability.

- **Modular Skill Framework**  
  AviScript-based DSL for creating and managing assistant behaviors.

- **Voice-First Architecture**  

- **Cross-Device Enclosure Support**  
  Deploy the `avi-enclosure` on any compatible IoT, edge device, or satellite node.

- **Flexible Interfaces**
    - Command-Line
    - Web Interface (coming soon)
    - Voice Interface
    - API Layer (modular)

- **Contextual Awareness**  
  Persistent conversation state and contextual command processing.

- **Async Event System**  
  Uses a lightweight messagebus for inter-module and cross-device communication (will transition to **Core** backend).

- **Multi-Language Support**  
  Current support: English 🇺🇸, Portuguese 🇵🇹🇧🇷  

- **Psychological & Emotional Layer**  
  Built on modernized ELIZA-style interaction for mental health support.

---

## 📦 Installation

> Prerequisites:
> - Rust (latest stable)

1. **Clone the repository:**
```bash
git clone https://github.com/YourUser/Avi
cd Avi
```

2. **Build the core system:**
```bash
cargo build --release
```

4. **Run Avi:**
```bash
# CLI Mode
./target/release/avi -m cli

# Voice Mode
./target/release/avi -m voice

# With GUI (when Avi GUI is available)
./target/release/avi -m gui

# Debug mode
./target/release/avi -d
```

---

## 🧠 Create Your Own Skills

Avi introduces **AviScript**, a domain-specific language for defining assistant behavior.

Use **Avi Studio** (coming soon) – a visual, drag-and-drop IDE – to build and debug skills.

Docs: [AviScript Guide (WIP)](docs/aviscript.md)

---

## 🧩 Ecosystem

- **avi-core** – Rust-powered assistant engine
- **avi-enclosure** – Lightweight binary for embedded devices
- **avi-gui** – Desktop or web interface for Avi interaction
- **avi-studio** – IDE for skill creation with AviScript
- **avi-audio** – Voice and audio infrastructure

---

## 🛠 Architecture

- `Core Engine`: Processes voice/input → recognizes intent → executes AviScript skills
- `Skill System`: Modular DSL-based system with hot-reloading
- `Interfaces`: CLI, GUI, Web, API
- `Event Bus`: Async communication over messagebus
- `Context Manager`: Remembers states, preferences, and session data
- `Translation Layer`: Dynamic i18n using DSL macros

---

## 🧪 Contributing

I ❤️ contributions. Jump in and build a skill, interface, or even a new enclosure:

1. Fork the repo
2. Create a feature branch
3. `cargo fmt && cargo clippy`
4. Commit and PR

---

## 📄 License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgements

- Inspired by [ELIZA](https://en.wikipedia.org/wiki/ELIZA), reimagined for the future
- Uses Neon modules by [Neon Gecko](https://github.com/neongeckocom)
- Successor of [ALEX](https://github.com/Apoll011/Alex)
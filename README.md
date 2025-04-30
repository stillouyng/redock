# redock 🌱
[![Release Build](https://github.com/stillouyng/redock/actions/workflows/release.yml/badge.svg)](https://github.com/stillouyng/redock/actions/workflows/release.yml)

**Redis Manager for macOS with Dock Integration**

A simple Rust application that **starts and stops the Redis server** when the app is opened or closed.
## 🚀 Usage
Just launch the app – Redis will start automatically. Close the app to stop Redis.

## ⚠️ Dependency
Before using **redock**, install Redis.
```bash
  brew install redis
```
After unzip use xattr to skip the malware check for this app.
```bash
  sudo xattr -rd com.apple.quarantine /Applications/Redock.app
```

## 📦 Installation
1. Download the latest version from [Releases](https://github.com/stillouyng/redock/releases).
2. Unzip and drag `Redock.app` to your `Applications` folder.  


## Evolution Highlights

### v0.1.1 - Core Foundation
- Implemented base functionality without GUI
- Basic Redis control functions

### v0.1.2 - GUI Prototype
- Added initial GUI interface
- Basic start/stop/ping controls *(Note: redis-cli detection not implemented yet)*

### v0.1.3 - Stability Improvements
- Added automatic redis-cli path detection
- Implemented Redis status tracking via AtomicBool
- Fixed critical launch issues

### v0.1.4 - Asynchronous Core Update
- Fully asynchronous operations
- Comprehensive error handling
- Brew/redis-cli existence verification
# redock 🌱
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
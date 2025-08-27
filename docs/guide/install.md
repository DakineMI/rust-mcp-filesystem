# Install

<!-- tabs:start -->

## Installation Methods

<!-- x-release-please-start-version -->

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-installer.sh | sh
```

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-installer.ps1 | iex"
```

<!-- x-release-please-end -->

```sh
brew install rust-mcp-stack/tap/rust-mcp-filesystem
```

| File | Platform | Checksum |
|------|----------|----------|
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-aarch64-apple-darwin.tar.gz](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-aarch64-apple-darwin.tar.gz)<!-- x-release-please-end --> | Apple Silicon macOS | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-aarch64-apple-darwin.tar.gz.sha256)<!-- x-release-please-end --> |
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-x86_64-apple-darwin.tar.gz](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-apple-darwin.tar.gz)<!-- x-release-please-end --> | Intel macOS | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-apple-darwin.tar.gz.sha256)<!-- x-release-please-end --> |
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-x86_64-pc-windows-msvc.zip](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-pc-windows-msvc.zip)<!-- x-release-please-end --> | x64 Windows (zip) | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-pc-windows-msvc.zip.sha256)<!-- x-release-please-end --> |
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-x86_64-pc-windows-msvc.msi](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-pc-windows-msvc.msi)<!-- x-release-please-end --> | x64 Windows (msi) | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-pc-windows-msvc.msi.sha256)<!-- x-release-please-end --> |
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-aarch64-unknown-linux-gnu.tar.gz](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-aarch64-unknown-linux-gnu.tar.gz)<!-- x-release-please-end --> | ARM64 Linux | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-aarch64-unknown-linux-gnu.tar.gz.sha256)<!-- x-release-please-end --> |
| <!-- x-release-please-start-version -->[rust-mcp-filesystem-x86_64-unknown-linux-gnu.tar.gz](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-unknown-linux-gnu.tar.gz)<!-- x-release-please-end --> | x64 Linux | <!-- x-release-please-start-version -->[checksum](https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.2.2/rust-mcp-filesystem-x86_64-unknown-linux-gnu.tar.gz.sha256)<!-- x-release-please-end --> |

<!-- tabs:end -->

### 📝 Important Notice

By default, **rust-mcp-filesystem** operates in **`read-only`** mode unless write access is explicitly enabled. To allow write access, you must include the **`-w`** or **`--write-access`** flag in the list of arguments in configuration.

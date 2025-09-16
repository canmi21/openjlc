# OpenJLC

<div align="center">
  <a href="https://github.com/canmi21/openjlc">
    <img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/openjlc.webp" alt="Logo" width="80" height="80">
  </a>

  <h3>OpenJLC</h3>
  <p>
    <a href="https://crates.io/crates/openjlc"><img src="https://img.shields.io/crates/dv/openjlc?color=cd9c80&labelColor=black&style=flat-square&logo=rust&label=Crates.io" /></a>
    <a href="https://aur.archlinux.org/packages/openjlc"><img src="https://img.shields.io/aur/version/openjlc?color=3498ca&labelColor=black&style=flat-square&logo=archlinux&logoColor=white&label=Aur" /></a>
    <a href="https://github.com/canmi21/openjlc/releases"><img src="https://img.shields.io/badge/Windows-11-blue?style=flat-square&labelColor=black&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJNMyAzaDguNTN2OC41M0gzek0xMi40NjkgM2g4LjUzdjguNTNoLTguNTN6TTMgMTIuNDdoOC41M1YyMUgzek0xMi40NjkgMTIuNDdoOC41M1YyMWgtOC41M3oiLz48L3N2Zz4%3D&logoColor=white" /></a>
    <a href="https://github.com/canmi21/openjlc/releases"><img src="https://img.shields.io/badge/MacOS-Sequoia-white?style=flat-square&labelColor=black&logo=apple&logoColor=white" /></a>
    <a href="https://github.com/canmi21/openjlc/releases"><img src="https://img.shields.io/badge/Linux-6.13.8-yellow?style=flat-square&labelColor=black&logo=linux&logoColor=white" /></a>
    <a href="https://github.com/canmi21/openjlc/stargazers"><img src="https://img.shields.io/github/stars/canmi21/openjlc?color=ffcb47&labelColor=black&style=flat-square&logo=github&label=Stars" /></a>
    <a href="https://github.com/canmi21/openjlc/releases"><img src="https://img.shields.io/github/downloads/canmi21/openjlc/total?color=9ac813&labelColor=black&logo=github&style=flat-square&label=Releases" /></a>
    <br />
    <br />
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/21c27d0a834b09710c25047a6c2dc004.png" alt="MacOS" width="99%"/></a>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-06-170618.webp" alt="Linux" width="49%"/></a>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-06-171116.webp" alt="Windows" width="49%"/></a>
    <br />
  </p>
</div>

As they say, your design workflow shouldn't be bogged down by tedious format conversions! OpenJLC simplifies Gerber to JLC conversion from AD, KiCad, and more. Effortlessly convert, share, and design.

## 👋🏻 Getting Started

### 入门指南

Whether for users or professional developers, OpenJLC will be your open information playground. Please be aware that OpenJLC is currently under active development, and feedback is welcome for any [issue](https://github.com/canmi21/openjlc/issues) encountered.

Feel free to try it using the following methods:

**请使用以下方法安装：**

<table>
  <tr><td>🚀 <b>Install via Cargo</b> <br> 通过 Cargo 安装</td><td><code>cargo install openjlc</code></td></tr>
  <tr><td>📦 <b>Arch Linux AUR</b> <br> Arch Linux AUR 安装</td><td>Install via: <code>yay -S openjlc</code> <br> 通过： <code>yay -S openjlc</code> 安装</td></tr>
  <tr><td>❇️ <b>Cross-platform builds</b> <br> 跨平台构建</td><td>Prebuilt binaries for Linux, Windows, macOS available on <a href="https://github.com/canmi21/openjlc/releases">releases</a> <br> Linux、Windows、macOS 的预构建二进制文件可在 <a href="https://github.com/canmi21/openjlc/releases">releases</a> 获取</td></tr>
  <tr><td>⚡️ <b>Run directly</b> <br> 直接运行</td><td><code>openjlc file.zip or path_to_file/file.zip</code></td></tr>
  <tr><td>🗂️ <b>Supported input formats</b> <br> 支持的输入格式</td><td>KiCad, Altium Designer, EasyEDA — and more on the way! <br> KiCad、Altium Designer、EasyEDA </td></tr>
  <tr><td>🛠️ <b>Gerber analyzer</b> <br> Gerber 分析器</td><td>Auto label output file with <code>-{detected-eda}-L{layer}</code> <br> 使用 <code>-{detected-eda}-L{layer}</code> 自动标记输出文件</td></tr>
  <tr><td>🔄 <b>Fast updates</b> <br> 快速更新</td><td>Track latest changes via <a href="https://github.com/canmi21/openjlc/actions?query=event%3Aworkflow_dispatch+branch%3Amain+is%3Asuccess">ci</a> <br> 通过 <a href="https://github.com/canmi21/openjlc/actions?query=event%3Aworkflow_dispatch+branch%3Amain+is%3Asuccess">ci</a> 跟踪最新更改</td></tr>
  <tr><td>📣 <b>Contribute / Feedback</b> <br> 贡献 / 反馈</td><td>Create issues, pull requests, or just ⭐ the repo to support us! <br> 创建问题、拉取请求，或只是 ⭐ 仓库来支持我们！</td></tr>
</table> 

## ⚡️ Lightning-Fast Speed

Powered by Rust's high performance, OpenJLC can convert an 8-layer PCB from Altium in under 500ms.

<div align="center">
  <p>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-06-222144.webp" alt="OpenJLC Speed Benchmark" width="99%"/></a>
  </p>
</div>

## 🧩 Uniform Output Naming

We provide powerful regular expression rules to auto-analyze and rename output files, giving you a clean and consistent output structure.

<div align="center">
  <p>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-06-221930.webp" alt="Auto Output Naming" width="99%"/></a>
  </p>
</div>

## 🛠️ Extra Drill File Handling

Unlike other conversion tools, OpenJLC supports auto-detection and classification of drill files — no manual steps required.

**Altium**
```yaml
Drill_NPTH_Through: "(?i).*slot\\s?h?oles.*\\.txt$"
Drill_PTH_Through: "(?i).*round\\s?h?oles.*\\.txt$"
Drill_PTH_Through_Via: "(?i)\\.REP$|.*via.*\\.txt$"
Drill_PTH_Through_GBR: "(?i)\\.GD1$"
Drill_PTH_Through_Via_GBR: "(?i)\\.GG1$"
```

**KiCad**
```yaml
Drill_PTH_Through: "(?i)(?!.*NPTH).*\\.DRL$"
Drill_PTH_Through_Via: "(?i).*\\bVIA\\b.*\\.DRL$"
Drill_NPTH_Through: "(?i).*\\bNPTH\\b.*\\.DRL$"
Drill_PTH_Through_GBR: "(?i)^[^N]*PTH[^N]*\\.GBR$"
Drill_PTH_Through_Via_GBR: "(?i).*\\bVIA\\b.*\\.GBR$"
Drill_NPTH_Through_GBR: "(?i).*\\bNPTH\\b.*\\.GBR$"
```

## 🖱️ Right-Click Integration on Windows

On Windows, OpenJLC supports seamless right-click processing for `.zip` Gerber archives. With just one click, you can get clean and correctly named outputs — even after processing by the Windows file system.

<div align="center">
  <p>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-07-004511.webp" alt="Windows Context Menu Integration - Step 1" width="99%"/></a>
    <a href="https://github.com/canmi21/openjlc" target="_blank"><img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-07-004544.webp" alt="Windows Context Menu Integration - Step 2" width="99%"/></a>
  </p>
</div>

## 📦 Easy GUI Installer for Windows

We offer a dedicated Windows GUI installer to simplify setup. With just a few clicks, you can install, update, or uninstall OpenJLC — all without touching the command line.

<div align="center"> <p> <a href="https://github.com/canmi21/openjlc" target="_blank"> <img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/5E7ABC2AB8CA3DCA8EA4E872AECB30F0.webp" alt="Windows Installer - Step 1" width="49%"/> </a> <a href="https://github.com/canmi21/openjlc" target="_blank"> <img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/CA8D9D363777D6D970035852BEC043DB.webp" alt="Windows Installer - Step 2" width="49%"/> </a> <a href="https://github.com/canmi21/openjlc" target="_blank"> <img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/6BC1EECDD7A4D6699F5F127B3843FFED.webp" alt="Windows Installer - Step 3" width="99%"/> </a> </p> </div>

## ❓ FAQ

### 常见问题解答

**Q: PowerShell says `openjlc` is not recognized as a command?**
**A:** This usually means the OpenJLC installation path is not added to your system's environment variables.
Please follow the instructions shown in the image below to add it. Restart PowerShell after applying the changes.

**Q: PowerShell 报错 `openjlc` 未被识别为命令？**
**A:** 因为 OpenJLC 安装路径未添加到您的系统环境变量中。请按照下图所示的说明添加它。应用更改后重新启动 PowerShell。

<div align="center">
  <p>
    <a href="https://github.com/canmi21/openjlc" target="_blank">
      <img src="https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/img/2025-04-07-104139.webp" alt="Add to System Environment Variables" width="99%"/>
    </a>
  </p>
</div>

**Q: What if I'm using Linux or macOS?**
**A:** Make sure the Cargo binary directory (usually `$HOME/.cargo/bin`) is added to your shell's environment variables.
For example, if you're using `bash`, `zsh` or `fish`, you can add the following line to your `~/.bashrc`, `~/.zshrc` `~/.config/fish/config.fish` file:

**Q: 如果我使用 Linux 或 macOS 呢？**
**A:** 确保 Cargo 二进制目录（通常为 `$HOME/.cargo/bin`）添加到您的 shell 环境变量中。例如，如果您使用 `bash`、`zsh` 或 `fish`，可以将以下行添加到您的 `~/.bashrc`、`~/.zshrc` 或 `~/.config/fish/config.fish` 文件中：

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Then reload your shell or run:
```bash
source ~/.bashrc   # or source ~/.zshrc
```

If you're using fish shell, add the path like this:
```fish
set -U fish_user_paths /home/canmi/.cargo/bin $fish_user_paths
```
And reload the config:
```fish
source ~/.config/fish/config.fish
```

## ⭐ Contribute

If you are just a user of OpenJLC, please give us a Star. If you’d like to participate in development, you can consider the following steps:
The main program is written in Rust:
```bash
cargo run
cargo build
cargo install --path .
```
The Windows installer is written in Go:
```bash
GOOS=windows GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-w64-mingw32-gcc go build -x -ldflags="-H=windowsgui" -o installer.exe
```

```bash
cargo run
cargo build
cargo install --path .
```

```bash
GOOS=windows GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-w64-mingw32-gcc go build -x -ldflags="-H=windowsgui" -o installer.exe
```

Additionally, there is an old version written in Python [here](https://github.com/canmi21/openjlc/tree/dev). The source code is still visible but is no longer maintained or used.

## ✨ Insights

![Alt](https://repobeats.axiom.co/api/embed/92d527ae3220e560f72f3a4bb9c9b24917ccb8fc.svg "Repobeats analytics image")

<a href="https://github.com/canmi21">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=canmi21/openjlc&type=Timeline&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=canmi21/openjlc&type=Timeline&type=Timeline" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=canmi21/openjlc&type=Timeline" />
  </picture>
</a>
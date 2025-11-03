# macOS 应用编译说明

## 概述

本文档详细说明如何将 Tauri 项目编译为 macOS 应用。项目使用 Tauri v2、Vue 3、TypeScript 和 Tailwind CSS 构建。

## 环境要求

### 操作系统
- macOS 10.15 或更高版本

### 开发工具
- Node.js (推荐 LTS 版本)
- Rust 工具链
- Xcode Command Line Tools

### 安装 Xcode Command Line Tools
如果尚未安装 Xcode Command Line Tools，请在终端中运行以下命令：
```bash
xcode-select --install
```

### 安装 Rust
如果尚未安装 Rust，请运行：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 项目设置

### 安装依赖
在项目根目录下运行以下命令安装项目依赖：
```bash
npm install
```

### 检查 Tauri 环境
确保 Tauri 环境配置正确：
```bash
npm run tauri info
```

## 编译 macOS 应用

### 开发模式运行
在开发模式下运行应用以进行测试：
```bash
npm run tauri dev
```

### 构建生产版本
构建生产版本的 macOS 应用：
```bash
npm run tauri build
```

这将执行以下步骤：
1. 运行 `beforeBuildCommand` (npm run build) 编译前端代码
2. 将编译后的前端代码打包到 `dist` 目录
3. 使用 Rust 编译后端代码
4. 将所有内容打包为 macOS 应用

### 仅构建 macOS 版本
如果只想构建 macOS 版本而不构建其他平台的版本，可以使用以下命令：
```bash
npm run tauri build -- --target universal-apple-darwin
```

## 配置说明

### 应用配置 (tauri.conf.json)
```json
{
  "productName": "tauri-app",
  "version": "0.1.0",
  "identifier": "com.hello.tauri-app",
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

关键配置项：
- `productName`: 应用名称
- `identifier`: 应用的唯一标识符（Bundle Identifier）
- `bundle.active`: 是否启用打包功能
- `targets`: 构建目标平台（"all" 表示所有支持的平台）
- `icon`: 应用图标文件列表，其中 icon.icns 是 macOS 专用图标

### Cargo.toml 配置
```toml
[package]
name = "tauri-app"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
edition = "2021"
```

## 输出文件

构建完成后，macOS 应用将位于以下目录：
```
src-tauri/target/release/bundle/macos/
```

其中包含：
- `.app` 文件：可直接运行的 macOS 应用
- `.dmg` 文件：用于分发的磁盘映像文件

## 应用分发

### DMG 文件
构建过程会自动生成 DMG 文件，可用于分发应用。用户可以下载并打开 DMG 文件，然后将应用拖拽到 Applications 文件夹中安装。

### 代码签名（可选）
对于分发到 App Store 或避免 Gatekeeper 警告，需要对应用进行代码签名：
```bash
npm run tauri build -- --config.tauri.bundle.macos.signingIdentity="Apple Development: YOUR_NAME (TEAM_ID)"
```

## 故障排除

### 常见问题

1. **缺少 Xcode Command Line Tools**
   - 错误信息：`xcode-select: error: tool 'xcodebuild' requires Xcode`
   - 解决方案：运行 `xcode-select --install`

2. **Rust 编译错误**
   - 错误信息：与 Rust 相关的编译错误
   - 解决方案：确保已安装 Rust 工具链并更新到最新版本
   ```bash
   rustup update
   ```

3. **权限问题**
   - 错误信息：与文件访问权限相关的错误
   - 解决方案：检查项目目录的读写权限

4. **前端构建失败**
   - 错误信息：TypeScript 或 Vite 相关错误
   - 解决方案：检查前端代码并确保所有依赖已正确安装
   ```bash
   npm install
   npm run build
   ```

### 日志查看
查看详细构建日志可以帮助诊断问题：
```bash
npm run tauri build -- --verbose
```

## 自定义配置

### 应用元数据
可以在 [tauri.conf.json](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/src-tauri/tauri.conf.json) 中修改以下元数据：
- `productName`: 应用名称
- `version`: 应用版本
- `identifier`: Bundle Identifier
- `app.windows.title`: 窗口标题

### 应用图标
替换 [src-tauri/icons](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/src-tauri/icons) 目录中的图标文件以使用自定义图标，确保包含 icon.icns 文件用于 macOS。

## 最佳实践

1. **构建前测试**
   - 在构建生产版本之前，先使用开发模式测试应用功能

2. **版本管理**
   - 在构建新版本之前更新 [tauri.conf.json](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/src-tauri/tauri.conf.json) 和 [Cargo.toml](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/src-tauri/Cargo.toml) 中的版本号

3. **清理构建**
   - 如果遇到构建问题，可以清理之前的构建缓存：
   ```bash
   cd src-tauri
   cargo clean
   ```

4. **依赖更新**
   - 定期更新依赖以获取最新功能和安全修复：
   ```bash
   npm update
   ```

## 相关资源

- [Tauri 官方文档](https://tauri.app/)
- [macOS 打包指南](https://tauri.app/v1/guides/building/macos/)
- [代码签名指南](https://tauri.app/v1/guides/distribution/sign-macos/)
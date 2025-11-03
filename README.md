# Tauri + Vue + TypeScript + Tailwind CSS

这是一个使用 Tauri、Vue 3、TypeScript 和 Tailwind CSS 构建的跨平台桌面应用程序模板。该模板使用 Vue 3 `<script setup>` 单文件组件，集成了 Tailwind CSS 以实现现代化的 UI 设计。

## 项目结构

```
.
├── README.md
├── index.html
├── package.json
├── postcss.config.js
├── tailwind.config.js
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── public/
├── src/
│   ├── assets/
│   │   ├── css/
│   │   │   └── tailwind.css
│   │   ├── main.css
│   │   └── vue.svg
│   ├── components/
│   │   └── TestComponent.vue
│   ├── App.vue
│   ├── main.ts
│   └── vite-env.d.ts
└── src-tauri/
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/
    ├── src/
    └── icons/
```

### 根目录文件

- `index.html`: 应用程序入口 HTML 文件
- `package.json`: Node.js 依赖和脚本配置
- `postcss.config.js`: PostCSS 配置文件，用于集成 Tailwind CSS
- `tailwind.config.js`: Tailwind CSS 配置文件
- `vite.config.ts`: Vite 构建工具配置文件
- `tsconfig.json`: TypeScript 配置文件

### src/ 目录

包含前端 Vue 应用程序源代码：

- `App.vue`: 主应用程序组件
- `main.ts`: 应用程序入口文件，初始化 Vue 应用并挂载到 DOM
- `components/`: Vue 组件目录
  - `TestComponent.vue`: 示例组件，展示 Tailwind CSS 样式用法
- `assets/`: 静态资源目录，包含样式文件和图像
  - `css/tailwind.css`: Tailwind CSS 指令文件
  - `main.css`: 全局样式文件
  - `vue.svg`: Vue logo 图标

### src-tauri/ 目录

包含 Tauri 桌面应用程序的 Rust 代码和配置：

- `Cargo.toml`: Rust 依赖配置
- `tauri.conf.json`: Tauri 应用程序配置
- `src/`: Rust 源代码
- `icons/`: 应用程序图标
- `capabilities/`: Tauri 权限配置

## 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 可用脚本

### 开发

```bash
# 启动开发服务器
npm run dev

# 启动 Tauri 开发模式
npm run tauri dev
```

### 构建

```bash
# 构建前端应用程序
npm run build

# 构建 Tauri 桌面应用程序
npm run tauri build
```

## 技术栈

- [Tauri](https://tauri.app/): 跨平台桌面应用程序框架
- [Vue 3](https://v3.vuejs.org/): 渐进式 JavaScript 框架
- [TypeScript](https://www.typescriptlang.org/): JavaScript 的超集，添加了静态类型
- [Vite](https://vitejs.dev/): 下一代前端构建工具
- [Tailwind CSS](https://tailwindcss.com/): 实用优先的 CSS 框架
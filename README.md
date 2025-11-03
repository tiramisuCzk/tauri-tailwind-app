# Tauri + Vue + TypeScript + Tailwind CSS

这是一个使用 Tauri、Vue 3、TypeScript 和 Tailwind CSS 构建的跨平台桌面应用程序模板。该模板使用 Vue 3 `<script setup>` 单文件组件，集成了 Tailwind CSS 以实现现代化的 UI 设计。

## 使用的技术版本

- Tauri: v2
- Vue: v3.5.13
- Tailwind CSS: v4.1.16
- Vite: v6.0.3
- TypeScript: v5.6.2

## 项目结构

```
.
├── README.md                     # 项目说明文档
├── index.html                    # 应用程序入口 HTML 文件，包含 Vue 挂载点
├── package.json                  # Node.js 项目配置文件，定义依赖和脚本命令
├── postcss.config.js             # PostCSS 配置文件，用于处理 Tailwind CSS
├── tailwind.config.js            # Tailwind CSS 配置文件，定义主题和内容路径
├── tsconfig.json                 # TypeScript 配置文件，定义编译选项
├── tsconfig.node.json            # Node.js 环境下的 TypeScript 配置文件
├── vite.config.ts                # Vite 构建工具配置文件，包含开发服务器设置
├── public/                       # 静态资源目录
│   └── tauri.svg                 # Tauri 图标文件
├── src/                          # 前端 Vue 应用程序源代码
│   ├── assets/                   # 静态资源目录，包含样式文件和图像
│   │   ├── css/
│   │   │   └── tailwind.css      # Tailwind CSS 指令文件，导入 Tailwind 的基础、组件和工具类
│   │   └── vue.svg               # Vue logo 图标
│   ├── components/               # Vue 组件目录
│   │   └── TestComponent.vue     # 示例组件，展示 Tailwind CSS 样式用法
│   ├── App.vue                   # 主应用程序组件，包含页面布局结构
│   ├── main.ts                   # 应用程序入口文件，初始化 Vue 应用并挂载到 DOM
│   └── vite-env.d.ts             # Vite 环境类型声明文件
└── src-tauri/                    # Tauri 桌面应用程序的 Rust 代码和配置
    ├── Cargo.toml                # Rust 项目的依赖和元数据配置文件
    ├── tauri.conf.json           # Tauri 应用程序配置文件，定义构建和运行时选项
    ├── build.rs                  # Rust 构建脚本
    ├── capabilities/             # Tauri 权限配置目录
    │   └── default.json          # 默认权限配置文件
    ├── src/                      # Rust 源代码目录
    │   ├── main.rs               # Tauri 应用程序入口文件
    │   └── lib.rs                # Tauri 命令处理模块，定义 Rust 函数供前端调用
    └── icons/                    # 应用程序图标目录，包含不同尺寸的图标文件
        ├── 32x32.png             # 32x32 像素的应用图标
        ├── 128x128.png           # 128x128 像素的应用图标
        ├── 128x128@2x.png        # 128x128 像素的高分辨率应用图标
        ├── icon.icns             # macOS 系统使用的图标文件
        └── icon.ico              # Windows 系统使用的图标文件
```

### 根目录文件说明

- `README.md`: 项目说明文档
- `index.html`: 应用程序入口 HTML 文件，包含 Vue 挂载点
- `package.json`: Node.js 项目配置文件，定义依赖和脚本命令
- `postcss.config.js`: PostCSS 配置文件，用于处理 Tailwind CSS
- `tailwind.config.js`: Tailwind CSS 配置文件，定义主题和内容路径
- `vite.config.ts`: Vite 构建工具配置文件，包含开发服务器设置
- `tsconfig.json`: TypeScript 配置文件，定义编译选项
- `tsconfig.node.json`: Node.js 环境下的 TypeScript 配置文件

### public/ 目录

- `tauri.svg`: Tauri 图标文件

### src/ 目录（前端 Vue 应用程序源代码）

- `App.vue`: 主应用程序组件，包含页面布局结构
- `main.ts`: 应用程序入口文件，初始化 Vue 应用并挂载到 DOM
- `components/`: Vue 组件目录
  - `TestComponent.vue`: 示例组件，展示 Tailwind CSS 样式用法
- `assets/`: 静态资源目录，包含样式文件和图像
  - `css/tailwind.css`: Tailwind CSS 指令文件，导入 Tailwind 的基础、组件和工具类
  - `vue.svg`: Vue logo 图标
- `vite-env.d.ts`: Vite 环境类型声明文件

### src-tauri/ 目录（Tauri 桌面应用程序的 Rust 代码和配置）

- `Cargo.toml`: Rust 项目的依赖和元数据配置文件
- `tauri.conf.json`: Tauri 应用程序配置文件，定义构建和运行时选项
- `build.rs`: Rust 构建脚本
- `capabilities/`: Tauri 权限配置目录
  - `default.json`: 默认权限配置文件
- `src/`: Rust 源代码目录
  - `main.rs`: Tauri 应用程序入口文件
  - `lib.rs`: Tauri 命令处理模块，定义 Rust 函数供前端调用
- `icons/`: 应用程序图标目录，包含不同尺寸的图标文件

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

- [Tauri](https://tauri.app/): 跨平台桌面应用程序框架，使用 Web 前端技术构建桌面应用
- [Vue 3](https://v3.vuejs.org/): 渐进式 JavaScript 框架，用于构建用户界面
- [TypeScript](https://www.typescriptlang.org/): JavaScript 的超集，添加了静态类型检查
- [Vite](https://vitejs.dev/): 下一代前端构建工具，提供快速的开发体验
- [Tailwind CSS](https://tailwindcss.com/): 实用优先的 CSS 框架，用于快速构建自定义设计
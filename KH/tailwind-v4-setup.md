# Tailwind CSS v4 配置与引入说明

## 概述

本文档详细说明了项目中 Tailwind CSS v4 的配置和引入方式。项目采用 Vue 3 + Vite + Tauri 技术栈，并成功集成了 Tailwind CSS v4。

## 版本信息

- Tailwind CSS: v4.1.16
- @tailwindcss/postcss: v4.1.16
- PostCSS: v8.5.6
- Autoprefixer: v10.4.21

## 安装依赖

项目中通过 npm 安装了以下相关依赖：

```json
{
  "devDependencies": {
    "@tailwindcss/postcss": "^4.1.16",
    "autoprefixer": "^10.4.21",
    "postcss": "^8.5.6",
    "tailwindcss": "^4.1.16"
  }
}
```

## 配置文件

### 1. PostCSS 配置 (`postcss.config.js`)

```js
export default {
  plugins: {
    '@tailwindcss/postcss': {},
    autoprefixer: {},
  }
}
```

注意：在 Tailwind CSS v4 中，我们使用 `@tailwindcss/postcss` 插件而不是旧版本的 `tailwindcss` 插件。

### 2. Tailwind 配置 (`tailwind.config.js`)

```js
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}"
  ],
  theme: {
    extend: {
      colors: {
        primary: '#3b82f6', // 添加一个自定义的 primary 颜色
      }
    },
  },
  plugins: [],
}
```

配置要点：
- `content` 指定了需要扫描的文件，包括 HTML 文件和所有 Vue、JS、TS 等文件
- 在 `theme.extend` 中添加了自定义颜色 `primary`

### 3. CSS 文件 (`src/assets/css/tailwind.css`)

```css
@import 'tailwindcss';
```

这是 Tailwind CSS v4 的新引入方式，比旧版本的 `@tailwind base; @tailwind components; @tailwind utilities;` 更简洁。

## 引入方式

在项目入口文件 `src/main.ts` 中引入 Tailwind CSS：

```ts
// 从 vue 模块中导入 createApp 函数，用于创建 Vue 应用实例
import { createApp } from "vue";
// 导入根组件 App.vue
import App from "./App.vue";
// 导入 Tailwind CSS 样式文件，使项目能够使用 Tailwind 的工具类
import './assets/css/tailwind.css'

// 创建 Vue 应用实例并挂载到 id 为 "app" 的 DOM 元素上
createApp(App).mount("#app");
```

## 使用方式

项目中直接使用 Tailwind CSS 的工具类，避免使用 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令：

```vue
<template>
  <div class="min-h-screen flex flex-col">
    <header class="bg-white shadow-sm py-4 px-6">
      <div class="container mx-auto flex justify-between items-center">
        <h1 class="text-2xl font-bold text-blue-500">Vue + Tailwind</h1>
        <!-- ... -->
      </div>
    </header>
    <!-- ... -->
  </div>
</template>
```

## 注意事项

### 1. 与旧版本的差异

Tailwind CSS v4 相比旧版本有以下重要变化：
1. 简化了配置方式
2. 改变了引入方式（使用 `@import 'tailwindcss';`）
3. 使用 `@tailwindcss/postcss` 插件替代原来的 `tailwindcss` 插件

### 2. 避免使用 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令

在 Tailwind CSS v4 中，推荐直接使用工具类而不是通过 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 创建组件类。

### 3. 与其他工具的兼容性

项目中的 PostCSS 和 Autoprefixer 与 Tailwind CSS v4 兼容良好，无需特殊配置。

## 常见问题与解决方案

### 1. 工具类未生效

确保：
- CSS 文件已正确引入到项目入口文件
- PostCSS 配置正确
- `tailwind.config.js` 中的 `content` 配置包含了所有需要扫描的文件

### 2. 自定义样式问题

在 Tailwind CSS v4 中，推荐通过 `tailwind.config.js` 的 `theme` 配置进行自定义，而不是使用 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令。

## 最佳实践

1. 直接使用 Tailwind CSS 提供的工具类
2. 通过 `tailwind.config.js` 进行主题扩展
3. 保持配置简洁，避免不必要的复杂性
4. 关注 Tailwind CSS 版本更新带来的变化
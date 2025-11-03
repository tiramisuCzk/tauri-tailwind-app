# Tailwind CSS v4 与 v3 配置和引入方式对比

## 概述

Tailwind CSS v4 相比 v3 版本进行了重大重构，引入了许多新特性和简化配置的方式。本文档将详细对比两个版本在配置和引入方式上的主要区别。

## 安装依赖

### Tailwind CSS v3

```bash
npm install -D tailwindcss@latest postcss@latest autoprefixer@latest
npx tailwindcss init -p
```

### Tailwind CSS v4

```bash
npm install -D tailwindcss@latest @tailwindcss/postcss@latest postcss@latest autoprefixer@latest
```

注意：v4 版本引入了新的 `@tailwindcss/postcss` 插件。

## 配置文件差异

### tailwind.config.js

#### Tailwind CSS v3

```js
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

#### Tailwind CSS v4

```js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      // 可以添加自定义配置
    },
  },
  plugins: [],
}
```

主要变化：
1. 使用 ES6 模块语法 (`export default`) 而不是 CommonJS (`module.exports`)
2. 添加了 JSDoc 类型注释

### postcss.config.js

#### Tailwind CSS v3

```js
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  }
}
```

#### Tailwind CSS v4

```js
export default {
  plugins: {
    '@tailwindcss/postcss': {},
    autoprefixer: {},
  }
}
```

主要变化：
1. 使用 ES6 模块语法 (`export default`) 而不是 CommonJS
2. 使用 `@tailwindcss/postcss` 插件替代了 `tailwindcss` 插件

## CSS 文件引入方式

### Tailwind CSS v3

在 CSS 文件中（如 [src/assets/css/tailwind.css](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/src/assets/css/tailwind.css)）：

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

### Tailwind CSS v4

在 CSS 文件中：

```css
@import 'tailwindcss';
```

或者在 JavaScript/TypeScript 文件中：

```js
import 'tailwindcss'
```

主要变化：
1. 大大简化了引入方式
2. 不再需要分别引入 `base`、`components` 和 `utilities`

## 使用方式差异

### Tailwind CSS v3

```vue
<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold mb-4">标题</h2>
    <p class="text-gray-600">内容</p>
  </div>
</template>
```

### Tailwind CSS v4

使用方式基本相同，但推荐做法有所变化：

```vue
<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold mb-4">标题</h2>
    <p class="text-gray-600">内容</p>
  </div>
</template>
```

## [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令的变化

### Tailwind CSS v3

```css
.card {
  @apply bg-white rounded-lg shadow-md p-6;
}
```

### Tailwind CSS v4

虽然仍然支持 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令，但官方更推荐直接使用工具类：

```css
/* 推荐：直接在 HTML/JSX 中使用工具类 */
/* 不推荐：使用 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 创建组件类 */
```

## 构建性能

### Tailwind CSS v3
- 构建时间较长
- 需要扫描所有内容文件以生成 CSS

### Tailwind CSS v4
- 构建速度显著提升
- 采用了新的引擎，性能优化明显
- 更智能的 CSS 生成机制

## 兼容性考虑

### Tailwind CSS v3
- 兼容性良好，稳定成熟
- 大量项目在使用

### Tailwind CSS v4
- 需要确保其他工具链兼容
- 某些旧的插件可能需要更新

## 项目迁移建议

如果要从 v3 迁移到 v4：

1. 更新依赖：
   ```bash
   npm uninstall tailwindcss
   npm install -D tailwindcss@latest @tailwindcss/postcss@latest
   ```

2. 修改 [postcss.config.js](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/postcss.config.js)：
   ```js
   export default {
     plugins: {
       '@tailwindcss/postcss': {},
       autoprefixer: {},
     }
   }
   ```

3. 简化 CSS 文件引入方式：
   ```css
   @import 'tailwindcss';
   ```

4. 更新 [tailwind.config.js](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/tailwind.config.js) 为 ES6 模块语法

## 总结

Tailwind CSS v4 相比 v3 版本的主要改进：

1. **配置简化**：更简洁的配置文件和引入方式
2. **性能提升**：构建速度显著提高
3. **现代化**：采用 ES6 模块语法
4. **插件更新**：使用新的 `@tailwindcss/postcss` 插件
5. **推荐实践**：更加强调直接使用工具类而非 [@apply](file:///Users/hello/Documents/03-code/AI-tauri/tauri-tailwind/node_modules/tailwindcss/screens.css) 指令

这些变化使得 Tailwind CSS v4 更加易于使用和维护，同时保持了与 v3 版本的大部分兼容性。
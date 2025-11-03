// 从 vue 模块中导入 createApp 函数，用于创建 Vue 应用实例
import { createApp } from "vue";
// 导入根组件 App.vue
import App from "./App.vue";
// 导入 Tailwind CSS 样式文件，使项目能够使用 Tailwind 的工具类
import './assets/css/tailwind.css'

// 创建 Vue 应用实例并挂载到 id 为 "app" 的 DOM 元素上
createApp(App).mount("#app");
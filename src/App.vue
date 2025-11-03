<template>
  <!-- 使用 flex 布局创建页面容器，最小高度为屏幕高度 -->
  <div class="min-h-screen flex flex-col">
    <!-- 导航栏 -->
    <header class="bg-white shadow-sm py-4 px-6">
      <!-- 使用容器类限制内容宽度并居中 -->
      <div class="container mx-auto flex justify-between items-center">
        <!-- 页面标题，使用蓝色主题和加粗字体 -->
        <h1 class="text-2xl font-bold text-blue-500">Vue + Tailwind</h1>
        <!-- 导航菜单 -->
        <nav>
          <!-- 使用 flex 布局创建水平导航项，项之间有间距 -->
          <ul class="flex gap-6">
            <!-- 导航链接，添加悬停效果 -->
            <li><a href="#" class="hover:text-blue-500 transition-colors">首页</a></li>
            <li><a href="#" class="hover:text-blue-500 transition-colors">关于</a></li>
            <li><a href="#" class="hover:text-blue-500 transition-colors">联系</a></li>
          </ul>
        </nav>
      </div>
    </header>

    <!-- 主内容区，使用 flex-grow 占据剩余空间 -->
    <main class="flex-grow container mx-auto px-6 py-8">
      <!-- 引入并使用测试组件 -->
      <TestComponent />
    </main>

    <!-- 页脚 -->
    <footer class="bg-gray-800 text-white py-6 px-6">
      <!-- 使用容器类限制内容宽度并居中 -->
      <div class="container mx-auto text-center">
        <!-- 版权信息 -->
        <p>© 2023 Vue + Tailwind 示例项目</p>
      </div>
    </footer>
  </div>
</template>

<script setup>
// 导入测试组件
import TestComponent from './components/TestComponent.vue'
</script>
<template>
  <div class="min-h-screen flex flex-col">
    <!-- 导航栏 -->
    <header class="bg-white shadow-sm py-4 px-6">
      <div class="container mx-auto flex justify-between items-center">
        <h1 class="text-2xl font-bold text-blue-500">Vue + Tailwind</h1>
        <nav>
          <ul class="flex gap-6">
            <li><a href="#" class="hover:text-blue-500 transition-colors">首页</a></li>
            <li><a href="#" class="hover:text-blue-500 transition-colors">关于</a></li>
            <li><a href="#" class="hover:text-blue-500 transition-colors">联系</a></li>
          </ul>
        </nav>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="flex-grow container mx-auto px-6 py-8">
      <TestComponent />
    </main>

    <!-- 页脚 -->
    <footer class="bg-gray-800 text-white py-6 px-6">
      <div class="container mx-auto text-center">
        <p>© 2023 Vue + Tailwind 示例项目</p>
      </div>
    </footer>
  </div>
</template>

<script setup>
import TestComponent from './components/TestComponent.vue'
</script>
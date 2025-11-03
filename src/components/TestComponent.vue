<template>
  <div class="space-y-8">
    <!-- 测试卡片 -->
    <div class="bg-white rounded-lg shadow-sm p-6 border border-gray-100">
      <h2 class="text-xl font-semibold mb-4">Tailwind 样式测试</h2>
      <p class="text-gray-600 mb-6">这是一个使用 Tailwind 工具类和自定义组件类的示例卡片</p>
      
      <!-- 动态样式演示 -->
      <div class="flex gap-4 mb-6">
        <button 
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-all"
          @click="isActive = !isActive"
        >
          切换状态
        </button>
        <div 
          :class="[
            'px-4 py-2 rounded-md',
            isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'
          ]"
        >
          {{ isActive ? '激活状态' : '默认状态' }}
        </div>
      </div>
      
      <!-- 响应式演示 -->
      <div class="border border-gray-200 rounded-md p-4">
        <p class="text-sm md:text-base lg:text-lg">
          响应式文本：小屏幕(sm)显示小字体，中等屏幕(md)显示中字体，大屏幕(lg)显示大字体
        </p>
        <div class="flex flex-col md:flex-row gap-2 mt-4">
          <div class="bg-blue-50 p-3 flex-1">项目1</div>
          <div class="bg-blue-50 p-3 flex-1">项目2</div>
          <div class="bg-blue-50 p-3 flex-1">项目3</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const isActive = ref(false) // 控制动态样式的状态
</script>
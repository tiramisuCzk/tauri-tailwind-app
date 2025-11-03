<template>
  <!-- 使用 space-y-8 为子元素添加垂直间距 -->
  <div class="space-y-8">
    <!-- 测试卡片 -->
    <div class="bg-white rounded-lg shadow-sm p-6 border border-gray-100">
      <!-- 卡片标题，使用 xl 字体大小、半粗体、底部间距 -->
      <h2 class="text-xl font-semibold mb-4">Tailwind 样式测试</h2>
      <!-- 卡片描述文本，使用灰色文字和底部间距 -->
      <p class="text-gray-600 mb-6">这是一个使用 Tailwind 工具类和自定义组件类的示例卡片</p>
      
      <!-- 动态样式演示区域，使用 flex 布局和间距 -->
      <div class="flex gap-4 mb-6">
        <!-- 切换状态按钮，定义了背景色、悬停效果、文字颜色、内边距、圆角和过渡效果 -->
        <button 
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-all"
          @click="isActive = !isActive"
        >
          切换状态
        </button>
        <!-- 状态显示区域，根据 isActive 的值动态切换背景色和文字颜色 -->
        <div 
          :class="[
            'px-4 py-2 rounded-md',                     // 固定样式：内边距和圆角
            isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'  // 根据状态切换颜色
          ]"
        >
          <!-- 根据 isActive 的值显示不同的文本 -->
          {{ isActive ? '激活状态' : '默认状态' }}
        </div>
      </div>
      
      <!-- 响应式演示区域，带有边框、圆角和内边距 -->
      <div class="border border-gray-200 rounded-md p-4">
        <!-- 响应式文本：在不同屏幕尺寸下显示不同字体大小 -->
        <p class="text-sm md:text-base lg:text-lg">
          响应式文本：小屏幕(sm)显示小字体，中等屏幕(md)显示中字体，大屏幕(lg)显示大字体
        </p>
        <!-- 响应式布局容器，移动端为垂直布局，中等屏幕及以上为水平布局 -->
        <div class="flex flex-col md:flex-row gap-2 mt-4">
          <!-- 三个示例项目，使用 flex-1 实现等宽分布 -->
          <div class="bg-blue-50 p-3 flex-1">项目1</div>
          <div class="bg-blue-50 p-3 flex-1">项目2</div>
          <div class="bg-blue-50 p-3 flex-1">项目3</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
// 从 vue 导入 ref 函数，用于创建响应式数据
import { ref } from 'vue'

// 创建一个名为 isActive 的响应式布尔值，默认为 false
// 用于控制动态样式的状态
const isActive = ref(false)
</script>
<template>
  <div class="common-layout" :class="{'dark': isDarkMode}">
    <el-container>
      <el-aside :class="{'is-collapse': isCollapse}">
        <div class="layout-container">
          <el-menu
            default-active="2"
            class="el-menu-vertical-demo"
            :collapse="isCollapse || windowWidth < 768"
            @mouseenter.native="handleCollapse(false)"
            @mouseleave.native="handleCollapse(true)"
          >
            <!-- A类模块 - 固定在顶部 -->
            <div class="menu-group menu-group-top">
              <el-menu-item index="about" class="A" disabled>
                <el-icon><icon-menu /></el-icon>
                <template #title>慕讯公益加速器</template>
              </el-menu-item>
              <el-menu-item index="feature" class="A" disabled>
                <template #title>永久免费 不玩套路</template>
              </el-menu-item>
              <el-menu-item index="version" class="A" disabled>
                <template #title>当前版本</template>
              </el-menu-item>
            </div>
            
            <!-- B类模块 - 在剩余空间中居中 -->
            <div class="menu-group menu-group-middle">
              <!-- 移除上下spacer元素 -->
              <el-menu-item index="2" class="B">
                <el-icon><icon-menu /></el-icon>
                <template #title>我的游戏</template>
              </el-menu-item>
              <el-menu-item index="3" class="B">
                <el-icon><document /></el-icon>
                <template #title>游戏库</template>
              </el-menu-item>
              <el-menu-item index="4" class="B">
                <el-icon><setting /></el-icon>
                <template #title>主机加速</template>
              </el-menu-item>
            </div>

            <el-menu-item index="5" class="B">
                <el-icon><setting /></el-icon>
                <template #title>自动展开</template>
              </el-menu-item>
          </el-menu>
        </div>
      </el-aside>
      <el-container>
        <el-header style="text-align: right; font-size: 12px; user-select: none;">
          <div class="toolbar" data-tauri-drag-region>
            <el-input
              v-model="input2"
              style="width: 240px"
              placeholder="搜索游戏"
              :suffix-icon="Search"
            />
            <el-button :icon="UserFilled" />
            <el-button-group>
              <el-button :icon="Bell" />
              <el-button :icon="Headset" />
              <el-button :icon="PictureFilled" />
              <el-button :icon="Setting" />
              <el-button :icon="isDarkMode ? Sunny : Moon" @click="toggleDarkMode" />
            </el-button-group>
            <el-button-group>
              <el-button :icon="Minus" @click="handleMinimize" />
              <el-button :icon="FullScreen" @click="handleToggleMaximize" />
              <el-button :icon="Close" @click="handleClose" />
            </el-button-group>
          </div>
        </el-header>
        <el-main>
          <main class="container">
            <h1>慕讯公益加速器</h1>

            <div class="row">
              <a href="https://vitejs.dev" target="_blank">
                <img src="/vite.svg" class="logo vite" alt="Vite logo" />
              </a>
              <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
              </a>
              <a href="https://vuejs.org/" target="_blank">
                <img src="../src/assets/icons/vue.svg" class="logo vue" alt="Vue logo" />
              </a>
            </div>
            <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

            <form class="row" @submit.prevent="greet">
              <input id="greet-input" v-model="name" placeholder="Enter a name..." />
              <button type="submit">Greet</button>
            </form>
            <p>{{ greetMsg }}</p>
          </main>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import {
    Document,
    Menu as IconMenu,
    Setting, Close, FullScreen, Minus, Headset, PictureFilled, UserFilled, Bell, Search, Moon, Sunny
  } from '@element-plus/icons-vue'
  import { invoke } from "@tauri-apps/api/core";
  import { Window } from '@tauri-apps/api/window';

  const greetMsg = ref("")
  const name = ref("")
  const input2 = ref("") // 补充定义 input2
  const isCollapse = ref(localStorage.getItem('sidebarCollapsed') === 'true')
  const windowWidth = ref(window.innerWidth)
  const isDarkMode = ref(false) //  添加深色模式状态

  async function greet() {
    greetMsg.value = await invoke("greet", { name: name.value });
  }


  onMounted(() => {
    const handleResize = () => {
      windowWidth.value = window.innerWidth
      if(windowWidth.value < 768) isCollapse.value = true
    }
    window.addEventListener('resize', handleResize)
    // 初始化深色模式
    const storedDarkMode = localStorage.getItem('darkMode')
    if (storedDarkMode) {
      isDarkMode.value = storedDarkMode === 'true'
    } else {
      // 默认跟随系统
      isDarkMode.value = window.matchMedia('(prefers-color-scheme: dark)').matches
    }
    updateDarkModeClass() // 应用深色模式类名

    // 添加清理函数
    onUnmounted(() => {
      window.removeEventListener('resize', handleResize)
    })
  })

  const handleCollapse = (state: boolean) => {
    if(windowWidth.value >= 768) {
      isCollapse.value = state
      localStorage.setItem('sidebarCollapsed', state.toString())
    }
  }
  const handleMinimize = async () => {
    try {
      const win = Window.getCurrent();
      await win.minimize();
    } catch (error) {
      console.error("最小化窗口失败:", error);
    }
  }

  const handleToggleMaximize = async () => {
    try {
      const win = Window.getCurrent();
      await win.toggleMaximize();
    } catch (error) {
      console.error("切换窗口最大化状态失败:", error);
    }
  }

  const handleClose = async () => {
    try {
      const win = Window.getCurrent();
      await win.close();
    } catch (error) {
      console.error("关闭窗口失败:", error);
    }
  }

  //  切换深色模式
  const toggleDarkMode = () => {
    isDarkMode.value = !isDarkMode.value
    localStorage.setItem('darkMode', String(isDarkMode.value)) //  存储深色模式状态
    updateDarkModeClass() //  更新 body 的 class
  }

  //  更新 body 的 class，应用或移除 'dark' 类
  const updateDarkModeClass = () => {
    if (isDarkMode.value) {
      document.body.classList.add('dark')
    } else {
      document.body.classList.remove('dark')
    }
  }
</script>

<style scoped>
  /* 之前的样式... */
  html, body, #app, .common-layout, .el-container {
    height: 100vh; /* 确保容器占满视口高度 */
    overflow: hidden; /* 防止出现滚动条 */
  }

  .common-layout > .el-container {
    display: flex; /* 使用 flex 布局使 aside 和主内容并排 */
    padding-left: 64px; /* 为固定侧边栏预留空间 */
  }

  /* 移除内层容器的内边距 */
  .el-container .el-container {
    padding-left: 0;
  }

  .el-aside {
    background-color: #f0f2f5; /* 示例背景色 */
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1); /* 优化动画曲线 */
    width: 64px; /* 默认折叠宽度 */
    position: fixed; /* 改为固定定位 */
    top: 0; /* 固定到顶部 */
    left: 0; /* 固定到左侧 */
    height: 100vh; /* 高度占满视口 */
    z-index: 100; /* 提高层级确保覆盖主内容 */
    overflow: hidden; /* 添加：隐藏溢出内容 */
    will-change: width; /* 添加：提示浏览器优化动画 */
  }

  /* 控制侧边栏宽度，根据 isCollapse 类 */
  .el-aside:not(.is-collapse) {
    width: 200px;
  }
  .el-aside.is-collapse {
    width: 64px;
  }

  .el-menu-vertical-demo {
    border-right: none;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
  }
  
  .menu-group {
    display: flex;
    flex-direction: column;
  }
  
  .menu-group-top {
    /* 固定顶部，不参与剩余空间分配 */
  }
  
  .menu-group-middle {
    /* 占据剩余空间并使内容居中 */
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center; /* 垂直居中对齐 */
  }
  
  /* 移除spacer样式 */
  .spacer {
    display: none;
  }

  .el-menu--collapse {
    width: 64px;
  }

  .el-menu:not(.el-menu--collapse) {
    width: 200px;
  }

  .el-header {
    background-color: #e9eef3; /* 示例背景色 */
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    cursor: grab;
    flex-shrink: 0;
    user-select: none;
  }
  .el-header:active {
    cursor: grabbing;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    justify-content: flex-end;
    height: 100%; /* 保持这一行 */
  }

  .layout-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .el-main {
    padding: 20px;
    overflow-y: auto;
    flex-grow: 1; /* 让主内容区域占据剩余空间 */
  }

  [data-tauri-drag-region] input,
  [data-tauri-drag-region] button,
  [data-tauri-drag-region] a,
  [data-tauri-drag-region] .el-menu-item,
  [data-tauri-drag-region] .el-input,
  [data-tauri-drag-region] .el-button,
  [data-tauri-drag-region] .el-button-group {
    cursor: default;
  }
</style>
<style lang="scss">
  @use "@/styles/main.scss" as main; // 添加命名空间别名

  // 提取公共样式混合宏
  @mixin dark-mode-bg($bg-color, $text-color: #ffffff) {
    background-color: $bg-color;
    color: $text-color;
  }

  body {
    margin: 0;
    padding: 0;
    border: none;
    border-radius: 0;
    overflow: hidden;
    background-color: #fff;
    color: #000; //  默认文字颜色
    transition: background-color 0.3s, color 0.3s; //  添加过渡效果
  }
  body.dark {
    background-color: #121212; //  深色背景
    color: #ffffff; //  深色模式文字颜色
  }

  .common-layout {
    border: none;
    border-radius: 0;
    overflow: hidden;
    height: 100vh;
  }
  .dark .el-header {
    background-color: main.$dark-header-bg; // 使用命名空间访问变量
    color: #ffffff;
  }
  .dark .el-aside {
    background-color: main.$dark-header-bg;
    color: #ffffff;
  }
  .dark .el-menu {
    background-color: main.$dark-header-bg;
    color: #ffffff;
  }
  .dark .el-menu-item:hover,
  .dark .el-menu-item.is-active {
    background-color: main.$dark-menu-active-bg !important;
  }
  .dark .el-input,
  .dark .el-input__inner {
    background-color: main.$dark-input-bg;
    color: #ffffff;
    border-color: main.$dark-border-color;
    box-shadow: 0 0 0 1px main.$dark-border-color;
  }
  .dark .el-button {
    color: #ffffff;
    background-color: main.$dark-button-bg;
    border-color: main.$dark-border-color;
  }
  .dark .el-button:hover {
    background-color: main.$dark-button-hover-bg;
    border-color: main.$dark-border-color;
  }

  /* 去除搜索框圆角 */
  .el-header .toolbar .el-input .el-input__inner {
    border-radius: 0 !important;
  }

  /* 禁止选择，但保留鼠标默认样式 */
  * {
    user-select: none;
    cursor: default; /* 设置默认鼠标样式 */
  }

  /* 允许交互元素的选择和鼠标样式 */
  input,
  textarea,
  button,
  a,
  .el-button,
  .el-input,
  .el-menu-item {
    user-select: auto;
    cursor: auto; /* 恢复默认鼠标样式 */
  }

  /* 针对 data-tauri-drag-region 内部的元素，保持默认鼠标样式 */
  [data-tauri-drag-region] button,
  [data-tauri-drag-region] a,
  [data-tauri-drag-region] .el-menu-item,
  [data-tauri-drag-region] .el-input,
  [data-tauri-drag-region] .el-button,
  [data-tauri-drag-region] .el-button-group {
    cursor: default;
  }

  /* 针对 el-header 内部的元素，保持默认鼠标样式 */
  .el-header button,
  .el-header a,
  .el-header .el-menu-item,
  .el-header .el-input,
  .el-header .el-button,
  .el-header .el-button-group {
    cursor: default;
  }

  /* 针对 el-aside 内部的元素，保持默认鼠标样式 */
  .el-aside button,
  .el-aside a,
  .el-aside .el-menu-item,
  .el-aside .el-input .el-input__suffix .el-icon {
    color: #aaaaaa;
  }
  .el-aside .el-button,
  .el-aside .el-button-group {
    cursor: default;
  }

</style>
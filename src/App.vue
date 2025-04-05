<template>
  <div class="common-layout">
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
            <el-menu-item index="1" class="A" disabled>
              <el-icon><icon-menu /></el-icon>
              <template #title>慕讯公益加速器</template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <template #title>永久免费 不玩套路</template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <template #title>当前版本</template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <template #title></template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <template #title></template>
            </el-menu-item>
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
    Setting, Close, FullScreen, Minus, Headset, PictureFilled, UserFilled, Bell, Search
  } from '@element-plus/icons-vue'
  import { invoke } from "@tauri-apps/api/core";
  import { Window } from '@tauri-apps/api/window';

  const greetMsg = ref("")
  const name = ref("")
  const input2 = ref("") // 补充定义 input2

  async function greet() {
    greetMsg.value = await invoke("greet", { name: name.value });
  }

  const isCollapse = ref(localStorage.getItem('sidebarCollapsed') === 'true')
  const windowWidth = ref(window.innerWidth)

  onMounted(() => {
    window.addEventListener('resize', () => {
      windowWidth.value = window.innerWidth
      if(windowWidth.value < 768) isCollapse.value = true
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
</script>

<style scoped>
  /* 之前的样式... */
  html, body, #app, .common-layout, .el-container {
    height: 100vh; /* 确保容器占满视口高度 */
    overflow: hidden; /* 防止出现滚动条 */
  }

  .el-container {
    display: flex; /* 使用 flex 布局使 aside 和主内容并排 */
  }

  .el-aside {
    background-color: #f0f2f5; /* 示例背景色 */
    flex-shrink: 0; /* 防止侧边栏在空间不足时被压缩 */
    transition: width 0.3s ease-in-out; /* 添加宽度变化的过渡效果 */
    width: 64px; /* 默认折叠宽度 */
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
  body {
    margin: 0;
    padding: 0;
    border: none;
    border-radius: 0;
    overflow: hidden;
    background-color: #fff;
  }

  .common-layout {
    border: none;
    border-radius: 0;
    overflow: hidden;
    height: 100vh;
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
  .el-aside .el-input,
  .el-aside .el-button,
  .el-aside .el-button-group {
    cursor: default;
  }

</style>
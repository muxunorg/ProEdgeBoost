<template>
  <div class="common-layout">
    <el-container>
      <!-- 在这里添加 data-tauri-drag-region -->
      <el-aside width="200px" data-tauri-drag-region>
        <div class="layout-container">
          <el-menu
            default-active="2"
            class="el-menu-vertical-demo"
            :collapse="isCollapse || windowWidth < 768"
            @mouseenter.native="handleCollapse(false)"
            @mouseleave.native="handleCollapse(true)"
          >
            <!-- 菜单项... -->
            <el-menu-item index="1" class="A" disabled>
              <el-icon><icon-menu /></el-icon>
              <template #title>慕讯公益加速器</template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <template #title>永久免费 不玩套路</template>
            </el-menu-item>
            <el-menu-item index="1" class="A" disabled>
              <el-icon><document /></el-icon>
              <template #title>当前版本</template>
            </el-menu-item>
            <!-- 菜单项 -->
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
            <el-menu-item index="5" class="B">
              <el-icon><CaretLeft v-if="isCollapse" /><CaretRight v-else /></el-icon>
              <template #title>{{ isCollapse ? '折叠' : '展开' }}</template>
            </el-menu-item>
          </el-menu>
        </div>
      </el-aside>
      <el-container>
        <!-- 在这里添加 data-tauri-drag-region -->
        <el-header style="text-align: right; font-size: 12px" data-tauri-drag-region>
          <div class="toolbar">
            <!-- 输入框和按钮... -->
            <el-input
              v-model="input2"
              style="width: 240px"
              placeholder="Please Input"
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
          <!-- 主要内容... -->
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

<!-- <script setup> 和 <style> 部分保持不变 -->
<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import {
    Document,
    Menu as IconMenu,
    Setting, CaretLeft, CaretRight, Close, FullScreen, Minus, Headset, PictureFilled, UserFilled, Bell, Search
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
    height: 97vh; /* 确保容器占满视口高度 */
    overflow: hidden; /* 防止出现滚动条 */
  }

  .el-aside {
    background-color: #f0f2f5; /* 示例背景色 */
    /* 可选：添加 grab 光标给用户视觉提示 */
    cursor: grab;
  }
  .el-aside:active {
    cursor: grabbing;
  }

  .el-header {
    background-color: #e9eef3; /* 示例背景色 */
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    /* 可选：添加 grab 光标给用户视觉提示 */
    cursor: grab;
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
  }

  .layout-container {
    height: 100%;
    display: flex;
    flex-direction: column;
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

  .el-main {
    padding: 20px;
    overflow-y: auto; /* 如果主内容可能超出，允许滚动 */
  }

  /* 确保拖动区域内的交互元素（按钮、输入框等）仍然是默认光标，可以正常点击 */
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
  // @use "@/styles/main.scss" as *;
  // .example {
  //   color: $primary-color;
  // }
</style>
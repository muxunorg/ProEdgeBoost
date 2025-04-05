<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import {
    Document,
    Menu as IconMenu,
    Setting, CaretLeft, CaretRight, Close, FullScreen, Minus, Headset, PictureFilled, UserFilled, Bell, Search
  } from '@element-plus/icons-vue'
  import { invoke } from "@tauri-apps/api/core";

  const greetMsg = ref("");
  const name = ref("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
  }

  // 侧边栏状态管理（网页3）
  const isCollapse = ref(localStorage.getItem('sidebarCollapsed') === 'true')
  const windowWidth = ref(window.innerWidth)

  onMounted(() => {
    window.addEventListener('resize', () => {
      windowWidth.value = window.innerWidth
      if(windowWidth.value < 768) isCollapse.value = true
    })
  })

  // 响应式处理（网页4）
  const handleCollapse = (state: boolean) => {
    if(windowWidth.value >= 768) {
      isCollapse.value = state
      localStorage.setItem('sidebarCollapsed', state.toString())
    }
  }
</script>

<template>
    <div class="common-layout">
      <el-container>
        <el-aside width="200px">
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
          <el-header style="text-align: right; font-size: 12px">
            <div class="toolbar">
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
                <el-button :icon="Minus" />
                <el-button :icon="FullScreen" />
                <el-button :icon="Close" />
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
<style scoped>
  /* src/styles/element.scss */
  :root {
    --el-menu-active-color: #409eff;
    --el-menu-bg-color: #ffffff;
    --el-menu-hover-bg-color: #f5f7fa;
  }

  /* 高度继承体系（网页1、网页4） */
  html, body, #app {
    height: 100%;
    overflow: hidden;
  }

  .layout-container {
    height: 97vh;
    display: flex;
  }

  .el-menu-vertical-demo {
    width: var(--menu-width);
    flex-direction: column;
  }

  .B {
    top: 15%;
  }

  .el-menu--collapse {
    --menu-width: 64px;
  }

  .el-menu:not(.el-menu--collapse) {
    --menu-width: 200px;
  }

</style>
<style lang="scss">
  @use "@/styles/main.scss" as *;
  .example {
    // 使用 main.scss 中定义的变量或样式
    color: $primary-color;
  }
</style>
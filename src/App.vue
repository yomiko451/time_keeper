<template>
  <div class="container">
    <Clock />
    <ClockMenu />
  </div>

</template>

<script lang="ts" setup>
import { ref, provide } from 'vue';
import Clock from './components/Clock.vue';
import ClockMenu from './components/Menu.vue';
import { injectSumSeconds, CountDownFlag, Theme } from './types'
import { invoke } from '@tauri-apps/api';

init()
document.oncontextmenu = () => { return false }
async function init() {
  await invoke('init')
  const theme: Theme = await invoke('get_theme')
  const globalStyle = document.documentElement.style
  globalStyle.setProperty('--background-color', theme.background_color)
  globalStyle.setProperty('--clock-board-color', theme.clock_board_color)
  globalStyle.setProperty('--button-background-color', theme.button_background_color)
  globalStyle.setProperty('--button-click-color', theme.button_click_color)
  globalStyle.setProperty('--menu-text-color', theme.menu_text_color)
  globalStyle.setProperty('--menu-font-family', theme.menu_font_family)
}


const sumSeconds = ref(0)
const countDownFlag = ref(CountDownFlag.Stop)

const getCountDownFlag = () => {
  return countDownFlag.value
}

const setCountDownFlag = (flag: CountDownFlag) => {
  countDownFlag.value = flag
}

const setSumSeconds = (second: number) => {
  sumSeconds.value = second
}
const getSumSeconds = () => {
  return sumSeconds.value
}

provide(injectSumSeconds, {
  getSumSeconds, 
  setSumSeconds,
  getCountDownFlag, 
  setCountDownFlag
  })
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  padding: 20px 0;
  box-sizing: border-box;
  align-items: center;
  row-gap: 20px;
  background-color: var(--background-color);
}
.menu {
  flex: 1;
  width: 90%;
}

</style>
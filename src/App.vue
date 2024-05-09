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
import { injectSumSeconds, CountDownFlag } from './types'

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
  padding: 10px;
  box-sizing: border-box;
  align-items: center;
}
.menu {
  flex: 1;
  height: 100%;
  width: 100%;
}

</style>
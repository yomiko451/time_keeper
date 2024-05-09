<template>
    <div class="menu">
        <div class="button minus" @click="minus"></div>
        <p class="time">{{ time }}</p>
        <div class="button add" @click="add"></div>
        <div class="button start" @click="startAndPause">{{ buttonStartLabel }}</div>
        <div class="button stop" @click="stop">{{ buttonStopLabel }}</div>
    </div>
</template>

<script lang="ts" setup>
import { ref, computed, inject, watch } from "vue";
import { injectSumSeconds, CountDownFlag } from '../types'

const { getCountDownFlag, setCountDownFlag, setSumSeconds, getSumSeconds } = inject(injectSumSeconds)!
const sumSeconds = ref(0)
const time = computed(() => {
    const hour = Math.floor(sumSeconds.value / 3600)
    const minute = Math.floor(sumSeconds.value / 60) % 60
    const second = sumSeconds.value % 60
    const hourStr = hour>9?hour:`0${hour}`
    const minuteStr = minute>9?minute:`0${minute}`
    const secondStr = second > 9 ? second : `0${second}`
    return `${hourStr}:${minuteStr}:${secondStr}`
})
const buttonStartLabel = ref("开始")
const buttonStopLabel = ref("停止")

const add = () => {
    if (sumSeconds.value < 60) {
        sumSeconds.value += 5
    } else if (sumSeconds.value < 300) {
        sumSeconds.value += 30
    } else if (sumSeconds.value < 3600) {
        sumSeconds.value += 300
    } else if (sumSeconds.value < 43200) {
        sumSeconds.value += 1800
    } else {
        sumSeconds.value = 43200
    }
    
}
const minus = () => {
    if (sumSeconds.value <= 0) {
        sumSeconds.value = 0
    } else if (sumSeconds.value <= 60) {
        sumSeconds.value -= 5
    } else if (sumSeconds.value <= 300) {
        sumSeconds.value -= 30
    } else if (sumSeconds.value <= 3600) {
        sumSeconds.value -= 300
    } else {
        sumSeconds.value -= 1800
    }
}

let temporarySumSeconds = 0
const startAndPause = () => {
    if (getCountDownFlag() === CountDownFlag.Stop) {
        setSumSeconds(sumSeconds.value * 1000)
        buttonStartLabel.value = "暂停"
        setCountDownFlag(CountDownFlag.Start)
    } else if (getCountDownFlag() === CountDownFlag.Start) {
        buttonStartLabel.value = "继续"
        temporarySumSeconds = getSumSeconds()
        setCountDownFlag(CountDownFlag.Pause)
    } else if (getCountDownFlag() === CountDownFlag.Pause) {
        setSumSeconds(temporarySumSeconds)
        buttonStartLabel.value = "暂停"
        setCountDownFlag(CountDownFlag.Resume)
    } else {
        buttonStartLabel.value = "继续"
        temporarySumSeconds = getSumSeconds()
        setCountDownFlag(CountDownFlag.Pause)
    }
}

const stop = () => {
    setCountDownFlag(CountDownFlag.Stop)
}

watch(getCountDownFlag, (newValue) => {
    if (newValue === CountDownFlag.Stop) {
        sumSeconds.value = 0
        buttonStartLabel.value = "开始"
        temporarySumSeconds = 0
    }
})

</script>

<style scoped>
.menu {
    display: flex;
    align-items: center;
    justify-content: space-evenly;
    margin: 0 10px;
}
.minus,.add {
    width: 40px;
    height: 40px;
    background-size: contain;
}
.start,.stop {
    width: 80px;
    height: 40px;
    line-height: 40px;
    font-size: 20px;
    text-align: center;
}
.button:hover,.time:hover {
    cursor: pointer;
    background-color: rgb(240, 240, 240);
}
.button:active {
    background-color: rgb(200, 200, 200);
}
.minus {
    background-image: url("../assets/left.svg");
}
.add {
    background-image: url("../assets/right.svg");
}
.time {
    font-size: 20px;
}
</style>
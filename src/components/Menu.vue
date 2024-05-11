<template>
    <div class="menu">
        <div class="button minus" @click="minus">减</div>
        <div class="time" @click="inputShow">{{ time }}
            <input type="text" maxlength="8" v-if="inputFlag" v-model="inputTime" ref="inputElement" @blur="inputHide"
                @keyup.enter="inputFlag = false">
        </div>
        <div class="button add" @click="add">加</div>
        <div class="button start" @click="startAndPause">{{ buttonStartLabel }}</div>
        <div class="button stop" @click="reset">复位</div>
    </div>
</template>

<script lang="ts" setup>
import { ref, computed, inject, watch, nextTick } from "vue";
import { injectSumSeconds, CountDownFlag } from '../types'
import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

const inputFlag = ref(false)
const inputTime = ref("00:00:00")
const inputElement = ref<HTMLInputElement | null>(null)

const inputShow = () => {
    if (getCountDownFlag() === CountDownFlag.Stop) {
        inputTime.value = "00:00:00"
        inputFlag.value = true
        nextTick(() => {
            inputElement.value?.focus()
            inputElement.value?.select()
        })
    }
}

const inputHide = () => {
    inputFlag.value = false
    const time = parseTime(inputTime.value)
    if (time > 0) {
        sumSeconds.value = time
    }
}

const parseTime = (time: string) => {
    const timeArr = time.split(":")
    const hour = parseInt(timeArr[0])
    const minute = parseInt(timeArr[1])
    const second = parseInt(timeArr[2])
    if (hour >= 0 && minute >= 0 && second >= 0) {
        return hour * 3600 + minute * 60 + second
    } else {
        message("请输入有效的时间格式！如：12:13:14", { title: "错误！", type: "error" })
        return 0
    }
}

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

const add = () => {
    if (sumSeconds.value < 3600) {
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
    } else if (sumSeconds.value <= 3600) {
        sumSeconds.value -= 300
    } else {
        sumSeconds.value -= 1800
    }
}

let temporarySumSeconds = 0
const startAndPause = () => {
    if (sumSeconds.value === 0) {
        return
    }
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

const reset = () => {
    if (getCountDownFlag() === CountDownFlag.Stop) {
        sumSeconds.value = 0
    } else {
        setCountDownFlag(CountDownFlag.Stop)
    }
}

watch(getCountDownFlag, (newValue) => {
    if (newValue === CountDownFlag.Stop) {
        if (getSumSeconds() <= 0) {
            invoke("play_sound")
        }
        sumSeconds.value = 0
        buttonStartLabel.value = "开始"
        temporarySumSeconds = 0
    }
})

watch(getSumSeconds, (newValue) => {
    if (newValue >= 0) {
        sumSeconds.value = Math.ceil(newValue / 1000)
    }
})
</script>

<style scoped>
.menu {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
input {
    position: absolute;
    top: 0;
    left: 0;
    outline: none;
    border: none;
    background-color: var(--background-color);
}
.minus,.add {
    width: 40px;
}
.start,.stop {
    width: 80px;
}
.minus,.add,.start,.stop {
    height: 40px;
    line-height: 40px;
    font-size: 20px;
    text-align: center;
    user-select: none;
}
.button:hover,.time:hover {
    cursor: pointer;
    background-color: var(--button-background-color);
}
.button:active {
    background-color: var(--button-click-color);
}
.time {
    font-size: 20px;
    position: relative;
    user-select: none;
}
input, .time {
    font-size: 20px;
    line-height: 40px;
    height: 40px;
    width: 90px;
    padding: 0 5px;
    box-sizing: border-box;
}
</style>
<template>
    <div class="clock">
        <canvas id="clock-board"></canvas>
    </div>
</template>

<script lang="ts" setup>
import { inject, watch, onMounted, ref } from "vue"
import { injectSumSeconds, CountDownFlag } from '../types'

const rect = ref({height: 320, width: 320})

onMounted(() => {
    const ratio = window.devicePixelRatio
    const canvas = document.getElementById('clock-board') as HTMLCanvasElement
    canvas.height = rect.value.height * ratio
    canvas.width = rect.value.width * ratio
    canvas.style.height = `${rect.value.height}px`
    canvas.style.width = `${rect.value.width}px`
    const ctx = canvas.getContext("2d")!
    ctx.scale(ratio, ratio)
    window.requestAnimationFrame(()=>animation(ctx))
    setInterval(() => {
        window.requestAnimationFrame(()=>animation(ctx))
    }, 250)
})


const globalStyle = window.getComputedStyle(document.documentElement)
const clockBoardColor = globalStyle.getPropertyValue('--clock-board-color')
const clockGraduationColor = globalStyle.getPropertyValue('--clock-graduation-color')
const clockHandColor = globalStyle.getPropertyValue('--clock-hand-color')
const clockSecondHandColor = globalStyle.getPropertyValue('--clock-second-hand-color')
const clockProgressColor = globalStyle.getPropertyValue('--clock-progress-color')

const { getCountDownFlag, setCountDownFlag, getSumSeconds, setSumSeconds } = inject(injectSumSeconds)!
let sumSeconds = 0
let graduation = 0


watch(getCountDownFlag, (newValue) => {
    if (newValue === CountDownFlag.Start) {
        sumSeconds = getSumSeconds() + Date.now()
        graduation = getSumSeconds() / (Math.PI * 2)
    } else if (newValue === CountDownFlag.Resume) {
        sumSeconds = getSumSeconds() + Date.now()
    }
})

let rotation = 0
function animation(ctx: CanvasRenderingContext2D) {
    const now = new Date()
    const hour = now.getHours() % 12
    const minute = now.getMinutes()
    const second = now.getSeconds()

    ctx.save()
    ctx.clearRect(0, 0, 320, 320)
    ctx.translate(160, 160)
    ctx.rotate(-Math.PI / 2)

    ctx.save()
    ctx.fillStyle = clockProgressColor
    if (getCountDownFlag() === CountDownFlag.Start || getCountDownFlag() === CountDownFlag.Resume) {
        const timeStamp = now.getTime()
        const deltaSumSeconds = sumSeconds - timeStamp
        setSumSeconds(deltaSumSeconds)
        rotation = deltaSumSeconds / graduation
    } else if (getCountDownFlag() === CountDownFlag.Pause) {
        rotation = getSumSeconds() / graduation
    } else {
        rotation = 0
    }
    if (rotation > 0) {
        ctx.beginPath()
        ctx.arc(0, 0, 160, 0, rotation)
        ctx.lineTo(0, 0)
        ctx.fill()
    } else {
        setCountDownFlag(CountDownFlag.Stop)
    }
    ctx.fillStyle = clockBoardColor
    ctx.beginPath()
    ctx.arc(0, 0, 150, 0, Math.PI * 2)
    ctx.fill()
    ctx.restore()

    ctx.save()
    ctx.strokeStyle = clockGraduationColor
    ctx.lineWidth = 6
    for (let i = 0; i < 12; i++) {
        ctx.beginPath()
        ctx.rotate(Math.PI / 6)
        ctx.moveTo(138, 0)
        ctx.lineTo(150, 0)
        ctx.stroke()
    }
    ctx.restore()

    ctx.save()
    ctx.strokeStyle = clockGraduationColor
    ctx.lineWidth = 4
    for (let i = 0; i < 60; i++) {
        ctx.beginPath()
        ctx.rotate(Math.PI / 30)
        ctx.moveTo(144, 0)
        ctx.lineTo(150, 0)
        ctx.stroke()
    }
    ctx.restore()

    ctx.save()
    ctx.rotate(
        (Math.PI / 6) * hour + (Math.PI / 360) * minute + (Math.PI / 21600) * second
    )
    ctx.strokeStyle = clockHandColor
    ctx.lineWidth = 8
    ctx.beginPath()
    ctx.moveTo(-20, 0)
    ctx.lineTo(75, 0)
    ctx.stroke()
    ctx.restore()

    ctx.save()
    ctx.rotate((Math.PI / 30) * minute + (Math.PI / 1800) * second)
    ctx.strokeStyle = clockHandColor
    ctx.lineWidth = 6
    ctx.beginPath()
    ctx.moveTo(-20, 0)
    ctx.lineTo(105, 0)
    ctx.stroke()
    ctx.restore()

    ctx.save()
    ctx.rotate((Math.PI / 30) * second)
    ctx.strokeStyle = clockSecondHandColor
    ctx.fillStyle = clockSecondHandColor
    ctx.lineWidth = 4
    ctx.beginPath()
    ctx.moveTo(-30, 0)
    ctx.lineTo(110, 0)
    ctx.stroke()
    ctx.beginPath()
    ctx.moveTo(110, -7.5)
    ctx.lineTo(135, 0)
    ctx.lineTo(110, 7.5)
    ctx.closePath()
    ctx.fill()
    ctx.beginPath()
    ctx.arc(0, 0, 7.5, 0, Math.PI * 2)
    ctx.fill()
    ctx.restore()
    
    ctx.restore()
}
</script>

<style scoped>
.clock {
    width: 320px;
    height: 320px;
    border-radius: 50%;
    background-color: var(--clock-board-color);
}

</style>
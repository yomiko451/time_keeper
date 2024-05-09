<template>
    <div class="clock">
        <canvas id="clock-board" width="320" height="320"></canvas>
    </div>
</template>

<script lang="ts" setup>
import { inject, watch } from "vue"
import { injectSumSeconds, CountDownFlag } from '../types'

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
function animation() {
    const now = new Date()
    const hour = now.getHours() % 12
    const minute = now.getMinutes()
    const second = now.getSeconds()
    const canvasElement = document.getElementById("clock-board") as HTMLCanvasElement;
    const ctx = canvasElement.getContext("2d")!

    ctx.save()
    ctx.clearRect(0, 0, 320, 320)
    ctx.translate(160, 160)
    ctx.rotate(-Math.PI / 2)

    ctx.save()
    ctx.fillStyle = "rgb(152,195,121)"
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
    ctx.fillStyle = "rgb(255,255,255)"
    ctx.beginPath()
    ctx.arc(0, 0, 150, 0, Math.PI * 2)
    ctx.fill()
    ctx.restore()

    ctx.save()
    ctx.lineWidth = 5
    for (let i = 0; i < 12; i++) {
        ctx.beginPath()
        ctx.rotate(Math.PI / 6)
        ctx.moveTo(130, 0)
        ctx.lineTo(150, 0)
        ctx.stroke()
    }
    ctx.restore()

    ctx.save()
    ctx.lineWidth = 2
    for (let i = 0; i < 60; i++) {
        ctx.beginPath()
        ctx.rotate(Math.PI / 30)
        ctx.moveTo(140, 0)
        ctx.lineTo(150, 0)
        ctx.stroke()
    }
    ctx.restore()

    ctx.save()
    ctx.rotate(
        (Math.PI / 6) * hour + (Math.PI / 360) * minute + (Math.PI / 21600) * second
    )
    ctx.lineWidth = 8
    ctx.beginPath()
    ctx.moveTo(-20, 0)
    ctx.lineTo(60, 0)
    ctx.stroke()
    ctx.restore()

    ctx.save()
    ctx.rotate((Math.PI / 30) * minute + (Math.PI / 1800) * second)
    ctx.lineWidth = 6
    ctx.beginPath()
    ctx.moveTo(-20, 0)
    ctx.lineTo(90, 0)
    ctx.stroke()
    ctx.restore()

    ctx.save()
    ctx.rotate((Math.PI / 30) * second)
    ctx.strokeStyle = "red"
    ctx.fillStyle = "red"
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
window.requestAnimationFrame(animation)
setInterval(() => {
    window.requestAnimationFrame(animation)
}, 250)
</script>

<style scoped>
.clock {
    width: 320px;
    height: 320px;
    border-radius: 50%;
    border: 2px solid black;
}

</style>
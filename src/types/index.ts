import { InjectionKey } from 'vue'

export enum CountDownFlag { 
    Start = 1,
    Pause = 2, 
    Resume = 3,
    Stop = 0
}

export const injectSumSeconds: InjectionKey<{
    getSumSeconds: () => number, 
    setSumSeconds: (second: number) => void,
    setCountDownFlag: (flag: CountDownFlag) => void,
    getCountDownFlag: () => CountDownFlag
}> = Symbol()
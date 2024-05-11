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

export interface Theme {
    background_color: string,
    clock_board_color: string,
    clock_graduation_color: string,
    clock_hand_color: string,
    clock_second_hand_color: string,
    clock_progress_color: string,
    button_background_color: string,
    button_click_color: string,
    menu_text_color: string,
    menu_font_family: string
}
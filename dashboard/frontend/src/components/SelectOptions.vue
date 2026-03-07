<template>
    <div class="select-options-root">
        <div class="select-options-container">
            <div
                class="select-options-main"
                :style="{ '--so-width': childWidth }"
            >
                <div
                    v-for="(data, idx) in props.data"
                    class="select-options"
                    @click="active = idx"
                >
                    <div
                        class="select-options-box"
                        :class="{ selected: active == idx }"
                    >
                        <label class="select-options-label"
                            ><span class="select-options-check"
                                ><input
                                    class="select-options-input"
                                    type="radio" /><span
                                    class="select-options-icons"
                                    ><svg
                                        class="select-options-icon-cycle"
                                        focusable="false"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z"
                                        ></path>
                                    </svg>
                                    <svg
                                        class="select-options-icon-pointer"
                                        focusable="false"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            d="M8.465 8.465C9.37 7.56 10.62 7 12 7C14.76 7 17 9.24 17 12C17 13.38 16.44 14.63 15.535 15.535C14.63 16.44 13.38 17 12 17C9.24 17 7 14.76 7 12C7 10.62 7.56 9.37 8.465 8.465Z"
                                        ></path>
                                    </svg> </span></span
                            ><span class="select-options-text">{{
                                data
                            }}</span></label
                        >
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

const props = defineProps({
    data: {
        type: Array as () => string[],
        default: () => [],
    },
});
const active = defineModel<number>('active', {
    default: 0,
});
const childWidth = computed(() => {
    return (1 / (props.data.length == 0 ? 1 : props.data.length)) * 100 + '%';
});
</script>

<style lang="css">
:root {
    --so-border-color: rgb(227, 232, 239);
    --so-text-color: rgb(0, 0, 0, 0.7);
}
:root.dark {
    --so-border-color: rgb(57, 57, 57);
    --so-text-color: rgba(255, 255, 255, 0.7);
}
.select-options-root {
    display: inline-flex;
    flex-direction: column;
    position: relative;
    min-width: 0px;
    padding: 0px;
    margin: 0px;
    border: 0px;
    vertical-align: top;
    flex: 1 1 0%;
    width: 100%;
}
.select-options-container {
    display: flex;
    flex-flow: wrap;
    margin-top: 8px;
    margin-bottom: 8px;
    border-radius: 4px;
}
.select-options-main {
    flex-flow: wrap;
    min-width: 0px;
    box-sizing: border-box;
    display: flex;
    gap: 24px;
    width: 100%;
}
.select-options {
    -webkit-box-flex: 0;
    flex-grow: 0;
    flex-basis: auto;
    width: calc(var(--so-width) - 12px);
    min-width: 0px;
    box-sizing: border-box;
}
.select-options-box.selected {
    border: 1px solid var(--main-color);
}
.select-options-box {
    border: 1px solid var(--so-border-color);
    border-radius: 4px;
    margin-right: 16px;
    padding-left: 8px;
    width: 100%;
}
.select-options-label {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    cursor: pointer;
    vertical-align: middle;
    -webkit-tap-highlight-color: transparent;
    margin-left: -11px;
    margin-right: 16px;
    align-self: flex-start;
    width: 100%;
}
.select-options-box.selected .select-options-check {
    color: var(--main-color);
}
.select-options-check {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    outline: 0px;
    border: 0px;
    margin: 0px;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    text-decoration: none;
    padding: 9px;
    border-radius: 50%;
    color: var(--so-text-color);
}
.select-options-input {
    cursor: inherit;
    position: absolute;
    opacity: 0;
    width: 100%;
    height: 100%;
    top: 0px;
    left: 0px;
    margin: 0px;
    padding: 0px;
    z-index: 1;
}
.select-options-icons {
    position: relative;
    display: flex;
}
.select-options-box.selected .select-options-icon-pointer {
    visibility: visible;
}
.select-options-icon-cycle {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.25rem;
    transform: scale(1);
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
.select-options-icon-pointer {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.25rem;
    left: 0px;
    position: absolute;
    transform: scale(1);
    transition: transform 150ms cubic-bezier(0, 0, 0.2, 1);
    visibility: hidden;
}
.select-options-text {
    margin: 0px;
    font-size: 14px;
    font-family: inherit;
    font-weight: 400;
    line-height: 1.5;
    text-wrap-mode: nowrap;
}
</style>

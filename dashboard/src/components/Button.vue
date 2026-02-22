<template>
    <button
        class="button"
        :class="{ reverse: props.reverseColor }"
        :type="props.type"
        ref="innerRef"
        :disabled="props.processing"
    >
        <Loading
            v-if="processing"
            class="btn-process"
            size="20px"
            color="var(--dark-1-color)"
        />
        <slot></slot>
    </button>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Loading from './Loading.vue';

const props = defineProps<{
    type?: 'button' | 'submit' | 'reset';
    processing?: boolean;
    reverseColor?: boolean;
}>();
const innerRef = ref<HTMLButtonElement>();
onMounted(() => {
    innerRef.value?.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            innerRef.value?.click();
        }
    });
});
</script>

<style>
:root {
    --btn-cbg: var(--main-color);
    --btn-disabled: rgba(0, 0, 0, 0.12);
}
:root.dark {
    --btn-cbg: var(--main-color);
    --btn-disabled: rgba(255, 255, 255, 0.12);
    /* color: var(--variant-textColor);
    background-color: var(--variant-textBg); */
}
:root .button:hover {
    --btn-cbg: rgb(10, 138, 135);
}
:root.dark .button:hover {
    --btn-cbg: rgb(170, 146, 125);
}
.button.reverse {
    color: var(--btn-cbg);
    background-color: transparent;
}

.button {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    font-family: inherit;
    font-weight: 500;
    font-size: 0.875rem;
    line-height: 1.75;
    text-transform: uppercase;
    min-width: 64px;
    color: var(--dark-1-color);
    background-color: var(--btn-cbg);
    box-shadow: none;
    width: 100%;
    outline: 0px;
    text-decoration: none;
    padding: 6px 16px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    border-radius: 4px;
    transition:
        background-color 250ms cubic-bezier(0.4, 0, 0.2, 1),
        box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1),
        border-color 250ms cubic-bezier(0.4, 0, 0.2, 1);
}
.button[disabled] {
    background-color: var(--btn-disabled);
    color: transparent;
}
.button:hover {
    box-shadow: none;
    text-decoration: none;
}
.btn-txt {
    overflow: hidden;
    pointer-events: none;
    position: absolute;
    z-index: 0;
    inset: 0px;
    border-radius: inherit;
}
.btn-process {
    margin: 10px;
    display: contents;
}
</style>

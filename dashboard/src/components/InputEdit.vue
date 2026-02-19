<template>
    <div class="inputedit-container" ref="container">
        <label class="inputedit-label" :for="`ie-${id}`">{{
            props.label
        }}</label>
        <div class="inputedit-main">
            <input
                class="inputedit-input"
                :type="props.type"
                :name="props.name"
                :value="value"
                :disabled="props.disabled"
                :required="props.required"
                :placeholder="placeholder"
                :id="`ie-${id}`"
                @focus="focus = true"
                @blur="focus = false"
                @input="handleInput"
            />
            <fieldset class="inputedit-fieldset">
                <legend class="inputedit-legend">
                    <span>{{ props.label }}--</span>
                </legend>
            </fieldset>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { isEmpty } from '../utils';
import { increaseInputID } from '../constant';
const id = increaseInputID();
const props = defineProps({
    label: {
        type: String,
        default: '',
    },
    placeholder: {
        type: String,
        default: '',
    },
    name: {
        type: String,
        default: '',
    },
    value: {
        type: String,
        default: '',
    },
    disabled: {
        type: Boolean,
        default: false,
    },
    required: {
        type: Boolean,
        default: false,
    },
    type: {
        type: String,
        default: 'text',
    },
});
const container = ref<HTMLDivElement>();
const value = ref<string>('');
const focus = ref<boolean>(false);
const placeholder = ref<string>(props.placeholder);
const emit = defineEmits(['update:value']);
defineExpose({
    focus: () => {
        container.value?.querySelector('input')?.focus();
    },
});

// 监听 input 事件并发射更新
const handleInput = (e: Event) => {
    const newValue = (e.target as HTMLInputElement).value;
    value.value = newValue;
    emit('update:value', newValue);
};
watch(
    [focus, value, props.placeholder],
    ([focus, value, ph]) => {
        const empty = isEmpty(value);
        if (focus) {
            container.value?.classList.add('inputedit-active');
            container.value?.classList.add('inputedit-focus');
            placeholder.value = (ph as string) || props.placeholder;
        } else {
            container.value?.classList.remove('inputedit-active');
            if (empty) {
                container.value?.classList.remove('inputedit-focus');
                placeholder.value = '';
            }
        }
    },
    { immediate: true },
);
</script>
<style>
:root {
    --inputedit-text-color: rgb(0, 0, 0);
    --inputedit-fieldset-border-color: rgba(0, 0, 0, 0.23);
}
:root.dark {
    --inputedit-text-color: rgb(255, 255, 255);
    --inputedit-fieldset-border-color: rgb(57, 57, 57);
}
.inputedit-active {
    color: var(--main-color);
}

.inputedit-container {
    display: inline-flex;
    flex-direction: column;
    position: relative;
    min-width: 0px;
    padding: 0px;
    margin: 0px;
    border: 0px;
    vertical-align: top;
    width: 100%;
}
.inputedit-focus .inputedit-label {
    user-select: none;
    pointer-events: auto;
    max-width: calc(133% - 32px);
    transform: translate(14px, -9px) scale(0.75);
}
.inputedit-active .inputedit-label {
    color: var(--main-color);
}
.inputedit-label {
    color: var(--text-color);
    font-size: 14px;
    font-family: inherit;
    font-weight: 400;
    line-height: 1.4375em;
    display: block;
    transform-origin: left top;
    text-overflow: ellipsis;
    position: absolute;
    left: 0px;
    top: 0px;
    z-index: 1;
    pointer-events: none;
    max-width: calc(100% - 24px);
    transform: translate(14px, 9px) scale(1);
    white-space: nowrap;
    overflow: hidden;
    transition:
        color 200ms cubic-bezier(0, 0, 0.2, 1),
        transform 200ms cubic-bezier(0, 0, 0.2, 1),
        max-width 200ms cubic-bezier(0, 0, 0.2, 1);
}
.inputedit-main {
    font-size: 14px;
    font-family: inherit;
    font-weight: 400;
    line-height: 1.4375em;
    color: var(--inputedit-text-color);
    box-sizing: border-box;
    cursor: text;
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    width: 100%;
    position: relative;
    border-radius: 4px;
}
.inputedit-input {
    font: inherit;
    letter-spacing: inherit;
    color: currentcolor;
    border: 0px;
    box-sizing: content-box;
    background: none;
    height: 1.4375em;
    margin: 0px;
    -webkit-tap-highlight-color: transparent;
    display: block;
    min-width: 0px;
    width: 100%;
    animation-name: mui-auto-fill-cancel;
    animation-duration: 10ms;
    padding: 8.5px 14px;
    outline: none;
}
.inputedit-main:hover .inputedit-fieldset {
    border-color: var(--main-color);
}
.inputedit-active .inputedit-fieldset {
    border-width: 2px;
    border-color: var(--main-color);
}
.inputedit-fieldset {
    text-align: left;
    position: absolute;
    inset: -5px 0px 0px;
    margin: 0px;
    padding: 0px 8px;
    pointer-events: none;
    border-radius: inherit;
    border-style: solid;
    border-width: 1px;
    overflow: hidden;
    min-width: 0%;
    border-color: var(--inputedit-fieldset-border-color);
}
.inputedit-legend {
    float: unset;
    width: auto;
    display: block;
    height: 11px;
    font-size: 0.75em;
    visibility: hidden;
    max-width: 0.01px;
    overflow: hidden;
    padding: 0px;
    transition: max-width 50ms cubic-bezier(0, 0, 0.2, 1);
    white-space: nowrap;
}
.inputedit-focus .inputedit-legend {
    max-width: 100%;
    transition: max-width 100ms cubic-bezier(0, 0, 0.2, 1) 50ms;
}
</style>

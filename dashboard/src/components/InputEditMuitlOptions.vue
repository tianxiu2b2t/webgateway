<template>
    <div class="inputedit-container" ref="container">
        <label class="inputedit-label" :for="`ie-${id}`">{{
            props.label
        }}</label>
        <div class="inputedit-main">
            <div
                class="MuiButtonBase-root MuiChip-root MuiChip-outlined MuiChip-sizeMedium MuiChip-colorDefault MuiChip-deletable MuiChip-deletableColorDefault MuiChip-outlinedDefault MuiAutocomplete-tag MuiAutocomplete-tagSizeMedium css-3pgfal"
                v-for="(data, idx) in values"
            >
                <span class="MuiChip-label MuiChip-labelMedium css-1fqh3rg">{{
                    data
                }}</span
                ><SvgIcon
                    class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium MuiChip-deleteIcon MuiChip-deleteIconMedium MuiChip-deleteIconColorDefault MuiChip-deleteIconOutlinedColorDefault css-1uuj6tr"
                    name="common-close"
                    size="0.6em"
                ></SvgIcon>
            </div>
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
            <div class="css-iuka1o" v-if="hasContent">
                <button
                    class="css-1622s5w"
                    tabindex="-1"
                    type="button"
                    aria-label="Clear"
                    title="Clear"
                >
                    <SvgIcon
                        class="icon"
                        name="common-close"
                        size="20px"
                    ></SvgIcon>
                </button>
            </div>
            <fieldset class="inputedit-fieldset">
                <legend class="inputedit-legend">
                    <span>{{ props.label }}--</span>
                </legend>
            </fieldset>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { isEmpty } from '../utils';
import { increaseInputID } from '../constant';
import SvgIcon from './SvgIcon.vue';
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
const values = ref<string[]>([]);
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
const hasContent = computed(() => {
    return !isEmpty(value.value) || !isEmpty(values.value);
});
watch(
    [
        () => focus.value,
        () => value.value,
        () => props.placeholder,
        () => hasContent.value,
    ],
    ([focus, _, ph, hasContent]) => {
        if (focus) {
            container.value?.classList.add('inputedit-active');
            container.value?.classList.add('inputedit-focus');
            placeholder.value = (ph as string) || props.placeholder;
        } else {
            container.value?.classList.remove('inputedit-active');
            if (hasContent) {
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

.css-1xog3j4 .MuiOutlinedInput-root .MuiAutocomplete-endAdornment {
    right: 9px;
}
.css-iuka1o {
    position: absolute;
    right: 0px;
    top: 50%;
    transform: translate(0px, -50%);
}
.icon {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.25rem;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
.css-1622s5w {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    text-align: center;
    font-size: 1.5rem;
    color: rgb(255, 255, 255);
    --IconButton-hoverBg: rgba(255, 255, 255, 0.08);
    visibility: hidden;
    outline: 0px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    margin: 0px -2px 0px 0px;
    text-decoration: none;
    flex: 0 0 auto;
    border-radius: 50%;
    transition: background-color 150ms cubic-bezier(0.4, 0, 0.2, 1);
    padding: 4px;
    right: 9px;
}
@media (pointer: fine) {
    .inputedit-container:hover .css-1622s5w {
        visibility: visible;
    }
}
.css-1xog3j4 .MuiAutocomplete-tag {
    margin: 3px;
    max-width: calc(100% - 6px);
}
.css-3pgfal {
    position: relative;
    -webkit-tap-highlight-color: transparent;
    user-select: none;
    appearance: none;
    max-width: 100%;
    font-family: inherit;
    font-size: 0.8125rem;
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    color: rgb(255, 255, 255);
    cursor: unset;
    vertical-align: middle;
    box-sizing: border-box;
    height: 24px;
    background-color: rgb(67, 67, 67);
    white-space: nowrap;
    transition:
        background-color 300ms cubic-bezier(0.4, 0, 0.2, 1),
        box-shadow 300ms cubic-bezier(0.4, 0, 0.2, 1);
    outline: 0px;
    text-decoration: none;
    padding: 0px;
    border-radius: 4px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    margin: 3px 4px;
}
.css-1fqh3rg {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 11px;
    padding-right: 11px;
}
.css-3pgfal .MuiChip-deleteIcon {
    margin-right: 5px;
}
.css-3pgfal .MuiChip-deleteIcon {
    -webkit-tap-highlight-color: transparent;
    color: rgba(255, 255, 255, 0.26);
    font-size: 22px;
    cursor: pointer;
    margin: 0px 5px 0px -6px;
}
.css-1uuj6tr {
    user-select: none;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.5rem;
    width: 0.6em;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
</style>

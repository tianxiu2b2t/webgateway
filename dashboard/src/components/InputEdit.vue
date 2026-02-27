<template>
    <div
        class="inputedit-container"
        :class="{ 'muitl-options': props.muitloptions }"
        ref="container"
    >
        <label class="inputedit-label" :for="`ie-${id}`">{{
            props.label
        }}</label>
        <div class="inputedit-main">
            <div class="inputedit-tag" v-for="(data, idx) in tags">
                <span class="inputedit-tag-label">{{ data }}</span
                ><SvgIcon
                    @click="deleteContent(idx)"
                    class="inputedit-tag-icon"
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
                @keydown.enter="handleEnter"
            />
            <div class="inputedit-cac" v-if="hasContent && props.muitloptions">
                <button
                    class="inputedit-cab"
                    tabindex="-1"
                    type="button"
                    aria-label="Clear"
                    title="Clear"
                    @click="clearAll"
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
                    <span>{{ props.label }}</span>
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
    tags: {
        type: Array as () => string[],
        default: () => [],
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
    muitloptions: {
        type: Boolean,
        default: false,
    },
});
const container = ref<HTMLDivElement>();
const value = ref<string>(props.value);
const tags = ref<string[]>(props.tags);
const focus = ref<boolean>(false);
const placeholder = ref<string>(props.placeholder);
const emit = defineEmits(['update:value', 'update:tags']);
defineExpose({
    focus: () => {
        container.value?.querySelector('input')?.focus();
    },
});

// 监听 input 事件并发射更新
function handleInput(e: Event) {
    const newValue = (e.target as HTMLInputElement).value;
    value.value = newValue;
    emit('update:value', newValue);
}
function handleEnter(e: Event) {
    if (!props.muitloptions) return;
    const newValue = (e.target as HTMLInputElement).value;
    if (isEmpty(newValue)) return;
    // find any same value
    if (tags.value.includes(newValue)) {
        // delete it
        tags.value = tags.value.filter((v) => v !== newValue);
    } else {
        // add it
        tags.value.push(newValue);
        value.value = '';
    }
    emit('update:value', newValue);
    emit('update:tags', tags.value);
}
const hasContent = computed(() => {
    return !isEmpty(value.value) || !isEmpty(tags.value);
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
            if (!hasContent) {
                container.value?.classList.remove('inputedit-focus');
                placeholder.value = '';
            }
        }
    },
    { immediate: true },
);
function deleteContent(idx: number) {
    tags.value.splice(idx, 1);
    emit('update:tags', tags.value);
}
function clearAll() {
    tags.value = [];
    value.value = '';
}
</script>
<style>
:root {
    --inputedit-text-color: rgb(0, 0, 0);
    --inputedit-fieldset-border-color: rgba(0, 0, 0, 0.23);
    --inputedit-mo-cab-color: rgba(0, 0, 0, 0.54);
    --inputedit-mo-cab-hover-color: rgba(0, 0, 0, 0.04);
    --inputedit-tag-color: rgb(0, 0, 0);
    --inputedit-tag-bg-color: rgb(233, 236, 240);
    --inputedit-tag-icon-color: rgba(0, 0, 0, 0.26);
}
:root.dark {
    --inputedit-text-color: rgb(255, 255, 255);
    --inputedit-fieldset-border-color: rgb(57, 57, 57);
    --inputedit-mo-cab-color: rgba(255, 255, 255, 0.54);
    --inputedit-mo-cab-hover-color: rgba(255, 255, 255, 0.08);
    --inputedit-tag-color: rgb(255, 255, 255);
    --inputedit-tag-bg-color: rgb(67, 67, 67);
    --inputedit-tag-icon-color: rgba(255, 255, 255, 0.26);
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
.muitl-options .inputedit-main {
    flex-wrap: wrap;
    padding-top: 6px;
    padding-bottom: 6px;
    padding-left: 6px;
    padding-right: 39px;
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
.muitl-options .inputedit-input {
    padding: 2.5px 4px 2.5px 8px;
    -webkit-box-flex: 1;
    flex-grow: 1;
    text-overflow: ellipsis;
    opacity: 0;
    width: 0px;
    min-width: 30px;
    opacity: 1;
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
.inputedit-legend > span {
    padding-left: 5px;
    padding-right: 5px;
    display: inline-block;
    opacity: 0;
    visibility: visible;
}
.inputedit-cac {
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
.inputedit-cab {
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
    color: var(--inputedit-mo-cab-color);
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
.inputedit-container:hover .inputedit-cab {
    visibility: visible;
}
.inputedit-active .inputedit-cab {
    visibility: visible;
}
.inputedit-cab:hover {
    background-color: var(--inputedit-mo-cab-hover-color);
}
.inputedit-tag {
    margin: 3px;
    max-width: calc(100% - 6px);
}
.inputedit-tag {
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
    color: var(--inputedit-tag-color);
    cursor: unset;
    vertical-align: middle;
    box-sizing: border-box;
    height: 24px;
    background-color: var(--inputedit-tag-bg-color);
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
.inputedit-tag-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 11px;
    padding-right: 11px;
}
.inputedit-tag-icon {
    user-select: none;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.5rem;
    width: 0.6em;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
    -webkit-tap-highlight-color: transparent;
    color: var(--inputedit-tag-icon-color);
    font-size: 22px;
    cursor: pointer;
    margin: 0px 5px 5px -6px;
}
</style>

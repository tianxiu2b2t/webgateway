<template>
    <div
        class="dfii-root"
        :class="{ dragging: dragging != 0 }"
        @dragenter.prevent="dragging++"
        @dragover.prevent="(e) => e.preventDefault()"
        @dragleave.prevent="dragging--"
        @drop.prevent="onDrop"
    >
        <div
            class="dfii"
            :style="{ width: `${width}px`, height: `${height}px` }"
        >
            拖入文件读取内容
        </div>
        <div class="dfii-slot" ref="slot">
            <slot></slot>
        </div>
    </div>
</template>
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

// 拖文件进到这里
const value = defineModel('value');
const slot = ref<HTMLDivElement>();
const observer = new ResizeObserver(refresh);
const width = ref(0);
const height = ref(0);
const dragging = ref(0);

function refresh() {
    width.value = slot.value?.clientWidth || 0;
    height.value = slot.value?.clientHeight || 0;
}

async function onDrop(e: DragEvent) {
    dragging.value = 0;

    const files = e.dataTransfer?.files;
    if (!files || files.length == 0) {
        return;
    }
    const file = files[0];
    const content = await file?.text();
    value.value = content;
}

onMounted(() => {
    observer.observe(slot.value!);
    refresh();
});
onUnmounted(() => {
    observer.disconnect();
});
</script>
<style lang="css">
.dfii-root {
    width: auto;
    height: auto;
}
.dfii-root.dragging .dfii {
    opacity: 1;
    visibility: visible;
}
.dfii {
    position: absolute;
    background-color: rgba(0, 0, 0, 0.4);
    z-index: 999;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    visibility: hidden;
    transition: opacity 200ms cubic-bezier(0, 0, 0.2, 1);
    cursor: pointer;
}
.dfii-slot {
    /* 插槽内容正常显示 */
    width: 100%;
    height: 100%;
}
</style>

<template>
    <div class="dfii-root">
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
import { onMounted, ref } from 'vue';

// 拖文件进到这里
const value = defineModel();
const slot = ref<HTMLDivElement>();
const observer = new ResizeObserver(refresh);
const width = ref(0);
const height = ref(0);

function refresh() {
    width.value = slot.value?.clientWidth || 0;
    height.value = slot.value?.clientHeight || 0;
    console.log(width.value, height.value);
}

onMounted(() => {
    observer.observe(slot.value!);
    refresh();
});
</script>
<style lang="css">
.dfii-root {
    width: auto;
    height: auto;
}
.dfii {
    position: absolute;
    background-color: rgba(0, 0, 0, 0.4);
    z-index: 999;
    display: flex;
    align-items: center;
    justify-content: center;
}
.dfii-slot {
    /* 插槽内容正常显示 */
    width: 100%;
    height: 100%;
}
</style>

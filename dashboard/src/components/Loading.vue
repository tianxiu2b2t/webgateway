<template>
    <div class="loading" :style="{ color: props.color }">
        <span
            class="loading-container"
            :style="{ height: props.size, width: props.size }"
        >
            <svg
                class="loading-svg"
                viewBox="22 22 44 44"
                width="100%"
                height="100%"
            >
                <circle
                    class="circle"
                    cx="44"
                    cy="44"
                    r="20.2"
                    fill="none"
                    stroke-width="3.6"
                />
            </svg>
        </span>
    </div>
</template>

<script setup lang="ts">
const props = defineProps({
    color: {
        type: String,
        default: 'currentColor',
    },
    size: {
        type: String,
        default: '1em',
    },
});
</script>

<style scoped>
.loading {
    position: absolute;
    display: flex;
    overflow: hidden;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
}

.loading-container {
    display: inline-block;
    animation: rotate 1.4s linear infinite;
}

.loading-svg {
    display: block;
}

.circle {
    stroke: currentColor;
    stroke-dasharray: 1px, 200px;
    stroke-dashoffset: 0;
    animation: dash 1.4s ease-in-out infinite;
}

@keyframes rotate {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg); /* 改为顺时针，与 Material-UI 一致 */
    }
}

@keyframes dash {
    0% {
        stroke-dasharray: 1px, 200px;
        stroke-dashoffset: 0;
    }
    50% {
        stroke-dasharray: 100px, 200px;
        stroke-dashoffset: -15px; /* 负偏移使弧段向前移动 */
    }
    100% {
        stroke-dasharray: 100px, 200px;
        stroke-dashoffset: -125px; /* 接近周长，产生完整旋转效果 */
    }
}
</style>

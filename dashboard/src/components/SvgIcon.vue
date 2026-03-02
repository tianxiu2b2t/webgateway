<template>
    <svg
        aria-hidden="true"
        :style="{
            width,
            height,
            color,
        }"
    >
        <use :xlink:href="`#icon-${name}`" :fill="color" />
    </svg>
</template>
<script setup lang="ts">
import { computed } from 'vue';

const width = computed(() =>
    props.size == 'default' ? undefined : props.size || props.width,
);
const height = computed(() =>
    props.size == 'default' ? undefined : props.size || props.height,
);
const props = defineProps({
    name: String,
    color: String,
    width: {
        type: String,
        default: '100%',
    },
    height: {
        type: String,
        default: '100%',
    },
    size: {
        type: String,
    },
});
</script>

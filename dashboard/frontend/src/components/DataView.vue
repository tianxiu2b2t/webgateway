<template>
    <div :title="`${props.data}`">{{ formatted }}</div>
</template>

<script setup lang="ts" generic="T">
import { computed } from 'vue';

const props = defineProps<{
    data: T;
    format: (data: T) => string | null | undefined;
}>();

const formatted = computed(() => props.format(props.data));
</script>

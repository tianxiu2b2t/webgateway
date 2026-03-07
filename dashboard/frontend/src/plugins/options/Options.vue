<template></template>
<script lang="ts" setup>
const props = defineProps({
    data: {
        type: Array as () => string[],
        default: () => [],
    },
    multioptions: {
        type: Boolean,
        default: false,
    },
});
</script>

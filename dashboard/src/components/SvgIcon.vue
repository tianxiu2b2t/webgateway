<template>
    <svg aria-hidden="true" :style="{ width, height, color }">
        <use :xlink:href="`#icon-${name}`" :fill="color" />
    </svg>
</template>
<script setup lang="ts">
defineProps({ name: String, color: String, width: String, height: String });
</script>

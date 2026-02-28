<template>
    <div class="backend">
        <InputEdit label="回源地址" />
        <InputEdit label="权衡" type="number" style="width: auto" />
    </div>
</template>

<script setup lang="ts">
import InputEdit from '../InputEdit.vue';
</script>

<style scoped>
.backend {
    display: flex;
    flex-direction: row;
    gap: 1rem;
}
</style>

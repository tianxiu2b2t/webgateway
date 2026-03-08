<template>
    <div class="backend">
        <InputEdit
            label="回源地址"
            placeholder="http://127.0.0.1:8080"
            v-model:value="url"
        />
        <InputEdit
            label="权衡"
            type="number"
            style="width: auto"
            :value="0"
            :v-model:value="balance"
        />
    </div>
</template>

<script setup lang="ts">
import InputEdit from '../InputEdit.vue';
const url = defineModel('url', {
    type: String,
    default: '',
});
const balance = defineModel('balance', {
    type: Number,
    default: 0,
});
</script>

<style scoped>
.backend {
    display: flex;
    flex-direction: row;
    gap: 1rem;
}
</style>

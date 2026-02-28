<script setup lang="ts">
import { ref, watch } from 'vue';
import InputEdit from '../../InputEdit.vue';
// expose
const key = ref('');
const secret = ref('');
const emit = defineEmits(['update:value']);
const props = defineProps({
    value: {
        type: Object,
        default: () => {},
    },
});
watch(
    () => [key.value, secret.value],
    ([key, secret]) => {
        console.log(key, secret);
        emit('update:value', { secret_id: key, secret_key: secret });
    },
);
// expose key and secret
</script>

<template>
    <InputEdit
        label="密钥 Key"
        placeholder="Secret Key"
        @update:value="(v) => (key = v)"
    />
    <InputEdit
        label="密钥 Secret"
        placeholder="Secret"
        @update:value="(v) => (secret = v)"
    />
</template>

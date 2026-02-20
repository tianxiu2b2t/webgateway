<template>
    <component
        v-for="value in dialogs"
        :is="value.component"
        @close="removeDialog(value.id)"
        :class="{ 'dialog-out-animation': value.out }"
    ></component>
</template>

<script lang="ts" setup>
import { dialogs, removeDialog } from '.';
// 监听所有component的emit close
</script>

<template>
    <Dialog v-for="value in dialogs"></Dialog>
</template>

<script lang="ts" setup>
import { dialogs } from '.';
import Dialog from './Dialog.vue';
</script>

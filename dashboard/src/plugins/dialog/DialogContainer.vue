<template>
    <component v-for="value in dialogs" :is="value"></component>
</template>

<script lang="ts" setup>
import { dialogs } from '.';
</script>

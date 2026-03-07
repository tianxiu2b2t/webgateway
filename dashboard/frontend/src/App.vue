<script setup lang="ts">
import DialogContainer from './plugins/dialog/DialogContainer.vue';
import Presentation from './plugins/presentation/Presentation.vue';
</script>
<template>
    <RouterView />
    <Presentation />
    <DialogContainer />
</template>

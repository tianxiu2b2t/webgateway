<template>
    <template v-if="props.type == 'icon'">
        <SvgIcon
            name="common-light"
            :height="props.size"
            :width="props.size"
            @click="toggleDark"
            v-if="!isDark"
        ></SvgIcon>
        <SvgIcon
            name="common-dark"
            :height="props.size"
            :width="props.size"
            @click="toggleLight"
            v-else
        ></SvgIcon
    ></template>
    <template v-else>
        <span>Light</span>
        <span>Dark</span>
    </template>
</template>

<script setup lang="ts">
import { isDark, toggleDark, toggleLight } from '../theme';
import SvgIcon from './SvgIcon.vue';

const props = defineProps({
    type: {
        type: String as () => 'icon' | 'text',
        default: 'icon',
    },
    size: {
        type: String,
        default: '100%',
    },
});
</script>
<style lang="css" scoped>
svg {
    cursor: pointer;
}
</style>

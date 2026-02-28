<template>
    <component
        v-for="value in dialogs"
        :is="value.component"
        @close="
            () => {
                value.props?.close?.();
                removeDialog(value.id);
            }
        "
        @cancel="
            () => {
                value.props?.cancel?.();
                removeDialogFromCancel(value.id);
            }
        "
        @confirm="
            () => {
                value.props?.confirm?.();
                removeDialogFromConfirm(value.id);
            }
        "
        :class="{ 'dialog-out-animation': value.out }"
        :v-bind="value.props"
    ></component>
</template>

<script lang="ts" setup>
import {
    dialogs,
    removeDialog,
    removeDialogFromCancel,
    removeDialogFromConfirm,
} from '.';
// 监听所有component的emit close
</script>

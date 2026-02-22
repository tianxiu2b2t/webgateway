<template>
    <Dialog>
        <template #header>添加站点</template>
        <template #content>
            <div class="content">
                <InputEditMuitlOptions label="匹配域名" />
                <InputEdit label="开放端口" />
                <InputEdit label="网站证书" />
                <div>
                    <AddWebsiteBackend />
                </div>
            </div>
        </template>
        <template #footer><DialogClose @cancel="cancel" /></template>
    </Dialog>
</template>

<script setup lang="ts">
import Dialog from '../../plugins/dialog/Dialog.vue';
import DialogClose from '../../plugins/dialog/DialogClose.vue';
import InputEdit from '../InputEdit.vue';
import InputEditMuitlOptions from '../InputEditMuitlOptions.vue';
import AddWebsiteBackend from './AddWebsiteBackend.vue';

const emit = defineEmits(['close']);

function cancel() {
    emit('close');
}
</script>

<style lang="css" scoped>
.content {
    width: 100%;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
}
</style>

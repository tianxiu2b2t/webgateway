<template>
    <Dialog>
        <template #header>添加站点</template>
        <template #content>
            <div class="content">
                <InputEdit
                    label="匹配域名"
                    :muitloptions="true"
                    placeholder="支持 * 以匹配网站域名"
                    v-model:tags="config.domains.value"
                />
                <InputEdit
                    label="开放端口"
                    :muitloptions="true"
                    v-model:tags="config.ports.value"
                />
                <InputEdit
                    label="网站证书"
                    placeholder="留空自动选择证书"
                    v-model:value="config.cert.value"
                />
                <div>
                    <AddWebsiteBackend />
                </div>
            </div>
        </template>
        <template #footer><DialogClose @cancel="cancel" /></template>
    </Dialog>
</template>

<script setup lang="ts">
import { reactive, ref, toRefs, watch } from 'vue';
import Dialog from '../../plugins/dialog/Dialog.vue';
import DialogClose from '../../plugins/dialog/DialogClose.vue';
import InputEdit from '../InputEdit.vue';
import AddWebsiteBackend from './AddWebsiteBackend.vue';
import { addDialog } from '../../plugins/dialog';
import DraftContent from '../../plugins/dialog/templates/DraftContent.vue';

const emit = defineEmits(['close']);
const state = reactive({
    ports: ['80', '443'],
    domains: ['*'],
    cert: '',
    backends: [],
});
const config = toRefs(state);

const modified = ref(false);
watch(
    () => [state.ports, state.domains, state.cert, state.backends],
    () => {
        modified.value = true;
    },
    { deep: true },
);

function cancel() {
    if (modified.value) {
        addDialog(DraftContent, {
            cancel: () => {
                console.log('cancel');
            },
            confirm: () => {
                emit('close');
            },
        });
        return;
    }
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

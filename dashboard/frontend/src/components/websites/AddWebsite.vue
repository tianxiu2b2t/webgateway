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
                    :muitloptions="true"
                    v-model:tags="config.cert.value"
                />
                <div v-for="(_, idx) in backends">
                    <AddWebsiteBackend v-model:result="backends[idx]" />
                </div>
            </div>
        </template>
        <template #footer
            ><DialogClose @cancel="cancel" @confirm="submit"
        /></template>
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
import { createWebsite } from '../../apis/websites';
import type {
    WebsiteBackendInput,
    WebsiteCreateRequest,
} from '../../types/websites';
import addPresentation from '../../plugins/presentation';

const emit = defineEmits(['close']);
const backends = ref<WebsiteBackendInput[]>([
    {
        url: '',
        balance: 0,
    },
]);
const state = reactive<{
    ports: string[];
    domains: string[];
    cert: string[];
    backends: WebsiteBackendInput[];
}>({
    ports: ['80', '443'],
    domains: ['*'],
    cert: [],
    backends: [
        {
            url: '',
            balance: 0,
        },
    ],
});
const config = toRefs(state);

const modified = ref(false);
watch(
    () => [state.ports, state.domains, state.cert, backends.value],
    () => {
        modified.value = true;
    },
    { deep: true },
);

function cancel() {
    if (modified.value) {
        addDialog(DraftContent, {
            confirm: () => {
                emit('close');
            },
        });
        return;
    }
    emit('close');
}
async function submit() {
    const data: WebsiteCreateRequest = {
        ports: state.ports.map((v) => parseInt(v)),
        hosts: state.domains,
        certificates: state.cert,
        backends: backends.value.map((v) => ({
            ...v,
            main: true,
        })),
    };
    const resp = await createWebsite(data);
    if (resp.status == 200) {
        addPresentation('添加成功', 'success');
        emit('close');
    } else {
        addPresentation(resp.message as string, 'alert');
    }
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

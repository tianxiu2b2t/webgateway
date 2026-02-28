<template>
    <Dialog>
        <template #header>添加证书</template>
        <template #content>
            <div class="content">
                <InputEdit
                    label="名称"
                    placeholder="仅作为标识使用"
                    @update:value="(v) => (name = v)"
                />
                <SelectOptions />
                <InputEdit label="证书公钥" :textarea="true" />
                <InputEdit label="证书私钥" :textarea="true" />
            </div>
        </template>
        <template #footer
            ><DialogClose @cancel="cancel" type="submit" @confirm="submit"
        /></template>
    </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import Dialog from '../../plugins/dialog/Dialog.vue';
import DialogClose from '../../plugins/dialog/DialogClose.vue';
import InputEdit from '../InputEdit.vue';
import { addDialog } from '../../plugins/dialog';
import DraftContent from '../../plugins/dialog/templates/DraftContent.vue';
import { create } from '../../apis/dnsproviders';
import addPresentation from '../../plugins/presentation';
import SelectOptions from '../SelectOptions.vue';

const emit = defineEmits(['close']);
const name = ref('');
const type = ref('tencent');
const tencent_result = ref({});
const domains = ref([]);

const modified = ref(false);
watch(
    () => [name.value, type.value, tencent_result.value],
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
function get_config() {
    if (type.value == 'tencent') {
        return tencent_result.value;
    }
    return {};
}
async function submit() {
    const resp = await create(
        name.value,
        domains.value,
        type.value,
        get_config(),
    );
    if (resp.status == 200) {
        addPresentation('添加成功', 'success');
        emit('close');
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

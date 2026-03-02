<template>
    <Dialog>
        <template #header>添加证书</template>
        <template #content>
            <div class="content">
                <InputEdit
                    label="名称"
                    placeholder="仅作为标识使用"
                    v-model:value="name"
                />
                <SelectOptions
                    :data="['自动签发', '手动上传']"
                    v-model:active="active"
                />
                <template v-if="active == 0">
                    <InputEdit
                        label="签发域名"
                        v-model:tags="domains"
                        :muitloptions="true"
                    />
                    <InputEdit label="域名解析" v-model="domains" />
                </template>
                <template v-if="active == 1">
                    <DragFileIntoInput
                        ><InputEdit label="证书公钥" :textarea="true"
                    /></DragFileIntoInput>
                    <DragFileIntoInput
                        ><InputEdit label="证书私钥" :textarea="true"
                    /></DragFileIntoInput>
                </template>
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
import SelectOptions from '../SelectOptions.vue';
import DragFileIntoInput from '../DragFileIntoInput.vue';

const emit = defineEmits(['close']);
const name = ref('');
const type = ref('tencent');
const active = ref(0);
const domains = ref([]);

const modified = ref(false);
watch(
    () => [name.value, type.value, active.value],
    () => {
        modified.value = true;
        console.log(active.value);
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
async function submit() {}
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

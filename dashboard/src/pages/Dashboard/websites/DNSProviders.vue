<template>
    <Table :config="tableConfig">
        <template #header
            ><div class="header">
                <div>
                    <span class="title">域名解析</span
                    ><span class="tip">为自动证书添加域名解析</span>
                </div>
                <Button style="width: auto" @click="addDialog(AddDNSProvider)"
                    >添加解析</Button
                >
            </div></template
        >
    </Table>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Table, { type TableConfig } from '../../../components/Table.vue';
import Button from '../../../components/Button.vue';
import { addDialog } from '../../../plugins/dialog';
import AddDNSProvider from '../../../components/websites/AddDNSProvider.vue';
const tableConfig = ref<TableConfig>({
    total: 0,
    headers: [
        {
            text: '名称',
            field: 'name',
        },
        {
            text: '类型',
            field: 'type',
        },
        {
            text: '域名',
            field: 'domain',
        },
        {
            text: '最后更新',
            field: 'updated_at',
        },
        {
            text: '创建时间',
            field: 'created_at',
        },
    ],
});
</script>

<style lang="css" scoped>
.header {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.tip {
    font-size: 14px;
    color: var(--text-color);
    margin-right: auto;
    margin-left: 16px;
}
</style>

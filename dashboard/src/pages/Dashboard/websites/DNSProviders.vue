<template>
    <Table :config="tableConfig"></Table>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Table, { type TableConfig } from '../../../components/Table.vue';
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

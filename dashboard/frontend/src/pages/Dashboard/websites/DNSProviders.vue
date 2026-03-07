<template>
    <Table
        :config="tableConfig"
        :data="data"
        @current-page="(v) => (currentPage = v)"
        @page-size="(v) => (perPage = v)"
    >
        <template #header
            ><div class="header">
                <div>
                    <span class="title">域名解析</span
                    ><span class="tip"
                        >(共
                        {{ tableConfig.total }}
                        个解析，为自动证书添加域名解析)</span
                    >
                </div>
                <Button style="width: auto" @click="addDialog(AddDNSProvider)"
                    >添加解析</Button
                >
            </div></template
        >
    </Table>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Table, { type TableConfig } from '../../../components/Table.vue';
import Button from '../../../components/Button.vue';
import { addDialog } from '../../../plugins/dialog';
import AddDNSProvider from '../../../components/websites/AddDNSProvider.vue';
import { fetch, total } from '../../../apis/dnsproviders';
import { formatDate } from '../../../utils';
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
const data = ref<any[]>([]);
const perPage = ref(10);
const currentPage = ref(1);
onMounted(async () => {
    await refresh();
});
async function refresh() {
    tableConfig.value.total = await total();
    const res = await fetch(currentPage.value - 1, perPage.value);
    data.value = res.map((v) => {
        return {
            name: v.name,
            type: v.type,
            // ','
            domain: v.domains.join(', '),
            updated_at: formatDate(v.updated_at),
            created_at: formatDate(v.created_at),
        };
    });
}
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

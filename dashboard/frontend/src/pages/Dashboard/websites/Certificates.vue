<template>
    <Table :config="tableConfig" :data="data"
        ><template #header
            ><div class="header">
                <div>
                    <span class="title">证书管理</span>
                    <span class="tip">
                        (共 {{ tableConfig.total }} 个证书)
                    </span>
                </div>
                <Button style="width: auto" @click="addDialog(AddCertificate)">
                    添加证书
                </Button>
            </div>
        </template>
    </Table>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Table, { type TableConfig } from '../../../components/Table.vue';
import { addDialog } from '../../../plugins/dialog';
import AddCertificate from '../../../components/websites/AddCertificate.vue';
import Button from '../../../components/Button.vue';
import { total, fetch } from '../../../apis/certificate';
import { formatDate } from '../../../utils';
const tableConfig = ref<TableConfig>({
    total: 0,
    headers: [
        {
            text: '名称',
            field: 'name',
        },
        {
            text: '域名',
            field: 'domain',
        },
        {
            text: '证书类型',
            field: 'type',
        },
        {
            text: '到期时间',
            field: 'expired_at',
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
            // ','
            type: v.dns_provider_id ? 'DNS' : 'Manual',
            domain: v.hostnames.join(', '),
            updated_at: formatDate(v.updated_at),
            created_at: formatDate(v.created_at),
            expired_at: formatDate(v.expires_at),
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

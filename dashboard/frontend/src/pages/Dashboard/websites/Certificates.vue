<template>
    <Table :config="tableConfig"
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
import { ref } from 'vue';
import Table, { type TableConfig } from '../../../components/Table.vue';
import { addDialog } from '../../../plugins/dialog';
import AddCertificate from '../../../components/websites/AddCertificate.vue';
import Button from '../../../components/Button.vue';
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

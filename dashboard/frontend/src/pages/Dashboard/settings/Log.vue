<template>
    <Table
        :config="logConfig"
        :data="data"
        @current-page="(v) => (currentPage = v)"
        @page-size="(v) => (perPage = v)"
    >
        <template #header>系统日志</template>
    </Table>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import Table from '../../../components/Table.vue';
import { fetchLog, getLogTotals, getUserInfo } from '../../../api';
import { formatDate } from '../../../utils';
const logConfig = ref({
    total: 0,
    headers: [
        {
            text: '用户',
            field: 'user',
        },
        {
            text: '操作内容',
            field: 'content',
        },
        {
            text: '操作时间',
            field: 'time',
        },
        {
            text: '操作IP',
            field: 'ip',
        },
    ],
});
const data = ref<object[] | null>(null);
const perPage = ref(10);
const currentPage = ref(1);
watch(
    () => currentPage.value,
    async () => {
        await refresh();
    },
);
async function refresh() {
    logConfig.value.total = await getLogTotals();
    const res = await fetchLog(perPage.value, currentPage.value - 1);
    data.value = await Promise.all(
        res.map(async (item) => {
            return {
                user: (await getUserInfo(item.user_id)).username,
                content: (() => {
                    const content = item.content;
                    if (content.type == 'raw') {
                        return content.content;
                    } else {
                        return content.content;
                    }
                })(),
                time: formatDate(item.created_at),
                ip: item.address,
            };
        }),
    );
}
onMounted(async () => {
    await refresh();
});
</script>

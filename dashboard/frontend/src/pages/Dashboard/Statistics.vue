<script setup lang="ts">
import { useRoute } from 'vue-router';
import Panel from '../../components/Panel.vue';
import Switchbar from '../../components/Switchbar.vue';
import { pushQuery } from '../../constant';
import { computed } from 'vue';
import Metrics from './statistics/metrics.vue';
import QPS from './statistics/QPS.vue';
const options = [
    {
        text: '近 24 小时',
        key: '1',
    },
    {
        text: '近 7 天',
        key: '7',
    },
    {
        text: '近 30 天',
        key: '30',
    },
];
const query = computed(() => useRoute().query);
</script>

<template>
    <Panel style="padding: 6px; margin-bottom: 24px">
        <Switchbar
            :data="options"
            @active="(v) => pushQuery({ in_days: v })"
            :active="options.findIndex((v) => query?.in_days == v.key)"
        />
    </Panel>
    <div class="statistics-overview">
        <Metrics :in_days="+(query?.in_days || 0)" />
        <QPS />
    </div>
</template>

<style>
.statistics-overview {
    display: flex;
    flex-wrap: wrap;
    gap: 24px;
    flex-wrap: nowrap;
}
</style>

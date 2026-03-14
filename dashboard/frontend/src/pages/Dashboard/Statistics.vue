<script setup lang="ts">
import { useRoute } from 'vue-router';
import Panel from '../../components/Panel.vue';
import Switchbar from '../../components/Switchbar.vue';
import { pushQuery } from '../../constant';
import { computed } from 'vue';
import Log from './settings/Log.vue';
import Metrics from './statistics/metrics.vue';
const options = [
    {
        text: '近 24 小时',
        key: '1d',
    },
    {
        text: '近 7 天',
        key: '7d',
    },
    {
        text: '近 30 天',
        key: '30d',
    },
];
const query = computed(() => useRoute().query);
</script>

<template>
    <Panel style="padding: 6px; margin-bottom: 24px">
        <Switchbar
            :data="options"
            @active="(v) => pushQuery({ time: v })"
            :active="options.findIndex((v) => query?.time == v.key)"
        />
    </Panel>
    <Metrics />
</template>

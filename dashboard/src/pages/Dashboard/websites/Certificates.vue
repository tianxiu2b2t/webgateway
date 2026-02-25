<script setup lang="ts">
import { useRoute } from 'vue-router';
import Panel from '../../../components/Panel.vue';
import Switchbar from '../../../components/Switchbar.vue';
import { pushQuery } from '../../../constant';
import { computed } from 'vue';
import DNSProviders from './DNSProviders.vue';
import Certs from './Certs.vue';
const options = [
    {
        text: '证书列表',
        key: 'certs',
    },
    {
        text: '域名解析',
        key: 'dns',
    },
];
const query = computed(() => useRoute().query);
</script>

<template>
    <Panel style="padding: 6px; margin-bottom: 24px">
        <Switchbar
            :data="options"
            @active="(v) => pushQuery({ tab: v })"
            :active="options.findIndex((v) => query?.tab == v.key)"
        />
    </Panel>
    <div v-if="query?.tab == 'dns'">
        <DNSProviders />
    </div>
    <div v-else>
        <Certs />
    </div>
</template>

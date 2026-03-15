<template>
    <div class="metrics-overview">
        <Panel class="metrics-view">
            <div>
                <PanelViewData>
                    <template #title> 请求次数 </template>
                    <template #value>
                        {{ data?.total_requests || 0 }}
                    </template>
                </PanelViewData>
            </div>
            <span></span>
            <div>
                <PanelViewData>
                    <template #title> 独立 IP </template>
                    <template #value> {{ data?.total_ips || 0 }} </template>
                </PanelViewData>
            </div>
        </Panel>
        <Panel class="metrics-view"
            ><div>
                <PanelViewData>
                    <template #title> 后端错误数 </template>
                    <template #value>
                        {{ data?.backend_error_requests || 0 }}
                    </template>
                </PanelViewData>
            </div>
            <span></span>
            <div>
                <PanelViewData>
                    <template #title> 后端错误率 </template>
                    <template #value>
                        {{
                            (
                                ((data?.total_requests || 0) == 0
                                    ? 0
                                    : (data?.backend_error_requests || 0) /
                                      (data?.total_requests || 0)) * 100
                            ).toFixed(2)
                        }}%
                    </template>
                </PanelViewData>
            </div>
        </Panel>
        <Panel class="metrics-view">
            <div>
                <PanelViewData>
                    <template #title> 4xx 错误数 </template>
                    <template #value>
                        {{ data?.e4xx_requests || 0 }}
                    </template>
                </PanelViewData>
            </div>
            <span></span>
            <div>
                <PanelViewData>
                    <template #title> 4xx 错误率 </template>
                    <template #value>
                        {{
                            (
                                ((data?.total_requests || 0) == 0
                                    ? 0
                                    : (data?.e4xx_requests || 0) /
                                      (data?.total_requests || 0)) * 100
                            ).toFixed(2)
                        }}%
                    </template>
                </PanelViewData>
            </div>
        </Panel>
        <Panel class="metrics-view">
            <div>
                <PanelViewData>
                    <template #title> 5xx 错误数 </template>
                    <template #value>
                        {{ data?.e5xx_requests || 0 }}
                    </template>
                </PanelViewData>
            </div>
            <span></span>
            <div>
                <PanelViewData>
                    <template #title> 5xx 错误率 </template>
                    <template #value>
                        {{
                            (
                                ((data?.total_requests || 0) == 0
                                    ? 0
                                    : (data?.e5xx_requests || 0) /
                                      (data?.total_requests || 0)) * 100
                            ).toFixed(2)
                        }}%
                    </template>
                </PanelViewData>
            </div>
        </Panel>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import Panel from '../../../components/Panel.vue';
import PanelViewData from '../../../components/PanelViewData.vue';
import { get_access_info } from '../../../apis/access';
import type { AccessInfo } from '../../../types/access';

const data = ref<AccessInfo>();
const task = ref();
const props = defineProps({
    in_days: {
        type: Number,
        default: 1,
    },
});
async function refreshInfo() {
    data.value = await get_access_info();
}
onMounted(async () => {
    refreshInfo();
    task.value = setInterval(refreshInfo, 60000);
});
onUnmounted(() => {
    clearInterval(task.value);
});
</script>

<style>
:root {
    --metrics-view-splitter-color: rgba(0, 0, 0, 0.2);
}
:root.dark {
    --metrics-view-splitter-color: rgba(255, 255, 255, 0.2);
}
</style>

<style scoped>
.metrics-view {
    display: flex;
    width: auto;
}
.metrics-view > div {
    flex: 8;
}
.metrics-view span {
    flex: 1;
    border-left: 0.2px solid var(--metrics-view-splitter-color);
}
.panel {
    min-width: none;
}
.metrics-overview > div {
    flex: 1;
}
.metrics-overview {
    display: flex;
    width: auto;
    min-width: auto;
    gap: 16px;
    flex-wrap: wrap;
    flex-direction: row;
}
</style>

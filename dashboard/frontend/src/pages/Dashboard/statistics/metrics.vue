<template>
    <Panel>
        <div>
            <p>请求次数</p>
            <p>1000</p>
        </div>
        <div>
            <p>独立 IP</p>
            <p>1000</p>
        </div>
        <div>
            <p>后端错误数</p>
            <p>1000</p>
        </div>
        <div>
            <p>后端错误率</p>
            <p>1000</p>
        </div>
        <div>
            <p>4xx 错误数</p>
            <p>1000</p>
        </div>
        <div>
            <p>4xx 错误率</p>
            <p>1000</p>
        </div>
        <div>
            <p>5xx 错误数</p>
            <p>1000</p>
        </div>
        <div>
            <p>5xx 错误率</p>
            <p>1000</p>
        </div>
    </Panel>
</template>

<script setup lang="ts">
import Panel from '../../../components/Panel.vue';

const props = defineProps({
    in_days: {
        type: Number,
        default: 1,
    },
});
</script>

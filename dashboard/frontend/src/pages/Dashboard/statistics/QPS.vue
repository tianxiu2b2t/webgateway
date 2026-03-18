<template>
    <Panel class="qps-root">
        <div class="title">
            <span>实时 QPS</span>
            <span>{{ data[data.length - 1]?.count || 0 }}</span>
        </div>
        <div class="value">
            <vchart :option="option" style="width: 100%; height: 100%" />
        </div>
    </Panel>
</template>

<script setup lang="ts">
import {
    computed,
    defineAsyncComponent,
    onMounted,
    onUnmounted,
    ref,
} from 'vue';
import Panel from '../../../components/Panel.vue';
import { get_qps } from '../../../apis/access';
import type { QPS } from '../../../types/access';
import { isDark } from '../../../theme';
import { darkMainColor, lightMainColor } from '../../../constant';

// 异步加载 ECharts 组件
const vchart = defineAsyncComponent(() => import('vue-echarts'));

// 响应式数据：存储已补全的 QPS 时间序列
const data = ref<QPS[]>([]);

// ECharts 配置项（基于 data 计算）
const option = computed(() => ({
    chartTheme: isDark.value ? 'dark' : 'light',
    color: isDark.value ? darkMainColor : lightMainColor,
    tooltip: { trigger: 'axis' },
    stateAnimation: {
        duration: 300,
        easing: 'cubicOut' as const,
    },
    grid: {
        top: 10,
        bottom: 10,
        right: 0,
        left: 0,
        show: false,
        z: 0,
        containLabel: false,
        backgroundColor: 'rgba(0,0,0,0)',
        borderWidth: 1,
        borderColor: '#ccc',
    },
    xAxis: {
        type: 'category',
        show: false,
        // 将 Date 对象转换为本地时间字符串作为轴标签（虽然隐藏，但 tooltip 会用）
        data: data.value.map((v) => v.time.toLocaleTimeString()),
        splitLine: {
            color: '#FFFFFF',
            type: 'dashed',
        },
    },
    yAxis: {
        show: false,
        type: 'value',
        splitLine: {
            color: '#FFFFFF',
            type: 'dashed',
        },
    },
    series: [
        {
            type: 'bar',
            name: 'QPS',
            data: data.value.map((v) => v.count),
            barGap: '0',
            barMinHeight: 4,
            itemStyle: {
                borderRadius: [2, 2, 0, 0],
            },
            z: 2,
            backgroundStyle: {
                color: 'rgba(180, 180, 180, 0.2)',
                borderColor: null,
                borderWidth: 0,
                borderRadius: 0,
                shadowBlur: 0,
            },
            select: {
                itemStyle: {
                    borderColor: '#212121',
                },
            },
        },
    ],
}));

// 定时器句柄
const task = ref<ReturnType<typeof setTimeout>>();

// 组件属性：显示的数据点数量（默认 60 个，对应 5 分钟）
const props = defineProps({
    count: {
        type: Number,
        default: 35,
    },
});

// 常量：时间窗口对齐参数（与后端约定一致）
const INTERVAL_MS = 5000; // 5 秒一个点
const OFFSET_MS = 2500; // 偏移 2.5 秒，使时间戳落在每个 5 秒区间的中间

/**
 * 刷新 QPS 数据：从 API 获取原始数据，补全缺失的时间点，更新 data
 */
async function refreshQPS() {
    try {
        const resp = (await get_qps()).data;
        const currentTime = new Date(
            Math.floor(+new Date(resp.current_time) / 5000) * 5000 - 5000,
        );
        const qps: QPS[] = [];
        const mapping: Map<number, number> = new Map();
        resp.data.forEach((item) => {
            mapping.set(+new Date(item.time), item.count);
        });
        for (let i = 0; i < props.count; i++) {
            const t = new Date(currentTime.getTime() - i * INTERVAL_MS);
            qps.push({
                time: t,
                count: mapping.get(+t) || 0,
            });
        }
        data.value = qps.reverse();
    } catch (error) {
        console.error('获取 QPS 数据失败', error);
        // 失败时不更新 data，保留旧数据
    }

    // 5. 安排下一次刷新，对齐到下一个“中间时刻”（与组件初始化逻辑一致）
    clearTimeout(task.value);
    const now = Date.now();
    const nextBoundary =
        Math.ceil((now - OFFSET_MS) / INTERVAL_MS) * INTERVAL_MS + OFFSET_MS;
    let delay = nextBoundary - now;
    if (delay < 0) delay = 0; // 防御性代码
    task.value = setTimeout(refreshQPS, delay);
}

// 组件挂载后立即开始刷新
onMounted(() => {
    refreshQPS();
});

// 组件卸载时清除定时器
onUnmounted(() => {
    clearTimeout(task.value);
});
</script>

<style scoped>
/* 确保 Panel 有足够的高度，否则图表可能不显示 */
.panel.qps-root {
    min-width: 0%;
    width: 412px;
    height: auto; /* 根据布局调整 */
}
.qps-root .title {
    font-size: 16px;
    font-weight: 500;
    display: flex;
    align-items: center;
    flex-wrap: nowrap;
    gap: 8px;
}
.qps-root .value {
    margin-top: 10px;
    height: calc(100% - 34px);
}
</style>

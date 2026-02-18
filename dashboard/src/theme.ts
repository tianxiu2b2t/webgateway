import { computed, onUnmounted, ref, watch } from 'vue';

// 系统深色模式媒体查询
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

// 初始化 dark 状态：优先从 localStorage 读取，若不存在则使用系统偏好
const stored = localStorage.getItem('dark');
const dark = ref(stored === null ? mediaQuery.matches : stored === 'true');

// 监听 dark 变化，更新 localStorage 和 HTML 类
watch(
    dark,
    (val) => {
        localStorage.setItem('dark', `${val}`);
        document.documentElement.classList.toggle('dark', val);
    },
    { immediate: true },
);

// 系统深色模式变化时的处理函数
function onSystemChange(e: MediaQueryListEvent) {
    // 只有当用户从未手动设置（localStorage 中无 'dark' 项）时才跟随系统
    if (localStorage.getItem('dark') === null) {
        dark.value = e.matches;
    }
}
mediaQuery.addEventListener('change', onSystemChange);

// 组件卸载时移除监听器
onUnmounted(() => {
    mediaQuery.removeEventListener('change', onSystemChange);
});

export function toggleDark() {
    dark.value = true;
}

export function toggleLight() {
    dark.value = false;
}

export const isDark = computed(() => dark.value);

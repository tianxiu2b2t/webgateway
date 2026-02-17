<template>
    <header ref="headerRef">
        <SvgIcon
            name="common-menu"
            color="var(--menu-color)"
            height="36px"
            width="36px"
        ></SvgIcon>
        <span class="spt-line"> <span class="spt-line-inner"> </span></span>
        <h2>Web Gateway</h2>
    </header>
    <div class="app-container">
        <div class="app-inner" :style="{ marginTop: `${headerHeight}px` }">
            <div class="app">
                <div class="side">
                    <Menu :menu-items="menu"></Menu>
                </div>
                <div class="main">
                    <main>
                        <RouterView></RouterView>
                    </main>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { getToken } from '../auth';
import SvgIcon from '../components/SvgIcon.vue';
import Menu, { type MenuItem } from '../components/Menu.vue';
const menu: MenuItem[] = [
    {
        title: '统计',
        to: 'statistics',
        icon: 'menu-statistics',
    },
    {
        title: '通用设置',
        to: 'settings',
        icon: 'menu-settings',
    },
];
const headerRef = ref<Element>();

const token = computed(() => getToken());
const headerHeight = ref(0);
console.log(headerHeight.value);
onMounted(() => {
    headerHeight.value = headerRef.value?.getBoundingClientRect().height || 0;
});
</script>
<style>
:root {
    --menu-color: rgb(35, 35, 35);
    --spt-line: rgba(0, 0, 0, 0.5);
}
:root.dark {
    --menu-color: rgb(250, 250, 250);
    --spt-line: rgba(255, 255, 255, 0.5);
}
</style>
<style scoped>
header {
    background-color: transparent;
    padding: 8px 12px;
    display: flex;
    align-items: center;
    color: var(--text-color);
    position: absolute;
    width: 100vw;
    z-index: 1000;
    left: 0;
    top: 0;
}
.spt-line {
    height: 100%;
    width: auto;
}
.spt-line-inner {
    height: 100%;
    min-height: 40px;
    border-left: 0.5px solid var(--spt-line); /* 左边框或右边框 */
    margin: 0px 8px;
}
.app-container {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
}
.app-inner {
    height: 100%;
    width: 100%;
}
.app {
    display: flex;
    height: 100%;
    width: 100%;
}
main {
    height: 100%;
}
.main {
    margin: 16px;
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}
.side {
    box-shadow: none;
    background-color: var(--bg-color);
    width: 228px;
    border-right: none;
    padding: 16px 12px 16px 24px;
}
</style>

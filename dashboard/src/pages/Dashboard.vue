<template>
    <header ref="headerRef">
        <div class="flex">
            <SvgIcon
                name="common-menu"
                color="var(--menu-color)"
                height="32px"
                width="32px"
                @click="toggleMenu"
                class="menu-btn"
            ></SvgIcon>
            <span class="spt-line"> <span class="spt-line-inner"> </span></span>
            <h2>Web Gateway</h2>
        </div>
        <div class="flex">
            <HeaderButton><Theme /></HeaderButton>
            <HeaderButton style="margin-left: 12px" @click="logout"
                ><SvgIcon
                    name="common-exit"
                    height="22px"
                    width="22px"
                ></SvgIcon
            ></HeaderButton>
        </div>
    </header>
    <div class="app-container">
        <div class="app-inner" :style="{ marginTop: `${headerHeight}px` }">
            <div class="app">
                <div class="side" ref="sideRef">
                    <Menu :menu-items="menu" :initRoutes="true"></Menu>
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
import { onMounted, ref } from 'vue';
import SvgIcon from '../components/SvgIcon.vue';
import Menu, { type MenuItem } from '../components/Menu.vue';
import Theme from '../components/Theme.vue';
import HeaderButton from '../components/HeaderButton.vue';
import { logout } from '../auth';
const menu: MenuItem[] = [
    {
        title: '统计',
        to: 'statistics',
        icon: 'menu-statistics',
    },
    {
        title: '网站管理',
        to: 'websites',
        icon: 'menu-website',
        subMenu: [
            {
                title: '网站列表',
                to: '',
            },
            {
                title: '证书管理',
                to: 'certificates',
            },
        ],
    },
    {
        title: '通用设置',
        to: 'settings',
        icon: 'menu-settings',
    },
];
const sideRef = ref<Element>();
const headerRef = ref<Element>();

const headerHeight = ref(0);
function toggleMenu() {
    sideRef.value?.classList.toggle('hide');
}
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
    justify-content: space-between;
    color: var(--text-color);
    position: absolute;
    width: 100vw;
    z-index: 1000;
    left: 0;
    top: 0;
}
header .flex {
    display: flex;
    height: 100%;
    align-items: center;
}
header .spt-line {
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
    margin-left: 228px;
    transition: margin-left 0.3s ease-in-out;
}
.side {
    box-shadow: none;
    background-color: var(--bg-color);
    width: 228px;
    border-right: none;
    padding: 16px 12px 16px 24px;
    position: absolute;
    height: 100%;
    transition: transform 0.3s ease-in-out;
}
.side.hide {
    transform: translateX(-100%);
}
.side.hide ~ .main {
    margin-left: 16px;
}
.menu-btn {
    cursor: pointer;
}
</style>

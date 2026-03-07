<template>
    <div class="menu-container">
        <template v-for="(item, idx) in menuItems">
            <div
                class="menu-item"
                v-if="item"
                @click="handleItem(idx)"
                :ref="
                    (el) => {
                        if (item.subMenu) {
                            subItemRefs[idx] = {};
                        }
                        menuItemRefs[idx] = el as HTMLDivElement;
                    }
                "
            >
                <div class="icon-container">
                    <SvgIcon
                        :name="item.icon"
                        class="icon"
                        v-if="item.icon"
                    ></SvgIcon>
                </div>
                <div class="text">
                    <span class="text-inner">{{ item.title }}</span>
                </div>
                <SvgIcon
                    name="menu-arrow"
                    class="arrow"
                    width=""
                    height=""
                    v-if="item.subMenu"
                ></SvgIcon>
            </div>
            <div
                class="subitem-container"
                style="min-height: 0px; height: 0; transition-duration: 237ms"
                v-if="item.subMenu"
                :ref="
                    (el) => (subItemContainerRefs[idx] = el as HTMLDivElement)
                "
            >
                <div class="subitem-inner">
                    <div class="subitems-inner-container">
                        <div
                            class="subitems"
                            v-for="(sub, sidx) in item.subMenu"
                        >
                            <div
                                class="subitem"
                                :ref="
                                    (el) =>
                                        ((subItemRefs[idx] as any)[sidx] =
                                            el as Element)
                                "
                                @click="handleSubItem(idx, sidx)"
                            >
                                <div class="subitem-icon">
                                    <div class="sub-menu-spot"></div>
                                </div>
                                <div class="subitem-text">
                                    <span class="text-inner"
                                        ><div class="subitem-text-inner">
                                            {{ sub.title }}
                                        </div></span
                                    >
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>

<style lang="css">
:root {
    --menu-bg-color: var(--main-color);
    --menu-text-color: rgb(255, 255, 255);
    --menu-box-shadow: rgba(15, 198, 194, 0.2) 0px 10px 25px 0px;
    --menu-text-7-color: var(--menu-text-7-color);
}
:root.dark {
    --menu-bg-color: linear-gradient(
        153deg,
        rgb(244, 209, 180) 0%,
        rgb(235, 187, 151) 100%
    );
    --menu-text-color: rgb(0, 0, 0);
    --menu-box-shadow: rgba(235, 187, 151, 0.2) 0px 10px 25px 0px;
    --menu-text-7-color: rgba(255, 255, 255, 0.7);
}
.menu-container {
    height: 100%;
}
.menu-item:hover {
    background-color: transparent;
    color: var(--main-color);
    text-decoration: none;
}
.menu-item.selected {
    background: var(--menu-bg-color);
    color: var(--menu-text-color);
    box-shadow: var(--menu-box-shadow);
    font-weight: 500;
}
@media screen and (min-height: 766px) {
    .menu-item {
        height: 50px;
    }
    .subitem {
        height: 44px;
    }
}
.menu-item .icon-container {
    width: 16px;
    height: 16px;
    min-width: 0px;
    margin-right: 12px;
}
.menu-item {
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    color: inherit;
    display: flex;
    -webkit-box-flex: 1;
    flex-grow: 1;
    -webkit-box-pack: start;
    justify-content: flex-start;
    -webkit-box-align: center;
    align-items: center;
    position: relative;
    min-width: 0px;
    box-sizing: border-box;
    text-align: left;
    padding-top: 8px;
    padding-bottom: 8px;
    padding-left: 16px;
    padding-right: 16px;
    height: 46px;
    outline: 0px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    margin: 0px 0px 4px;
    text-decoration: none;
    transition:
        color 200ms linear,
        background-color 200ms linear;
    border-radius: 4px;
}
.selected .icon-container {
    flex-shrink: 0;
    display: inline-flex;
    color: var(--menu-text-color);
}
.icon {
    width: 1em;
    height: 1em;
    fill: currentcolor;
}
.text {
    flex: 1 1 auto;
    min-width: 0px;
    margin-top: 4px;
    margin-bottom: 4px;
}
.selected .text {
    color: var(--menu-text-color);
}
.text-inner {
    margin: 0px;
    font-size: 14px;
    font-family: inherit;
    font-weight: 400;
    line-height: 1.5;
}
.arrow {
    width: 1em;
    height: 1em;
    fill: currentcolor;
    font-size: 18px;
    transform: rotate(-90deg);
}

.selected .arrow {
    width: 1em;
    height: 1em;
    fill: currentcolor;
    font-size: 18px;
    transform: rotate(180deg);
}

.subitem-container {
    height: auto;
    transition: height 300ms cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 237ms;
    overflow: hidden;
}
.subitem-inner {
    display: flex;
    width: 100%;
}
.subitems-inner-container {
    width: 100%;
}
.subitems {
    list-style: none;
    margin: 0px;
    padding: 0px;
    position: relative;
}
.subitem:hover {
    background-color: transparent;
    color: var(--main-color);
    text-decoration: none;
}

.subitem:hover .sub-menu-spot {
    background-color: var(--main-color);
}
.subitem.selected {
    background: transparent;
}

.subitem {
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    color: inherit;
    display: flex;
    -webkit-box-flex: 1;
    flex-grow: 1;
    -webkit-box-pack: start;
    justify-content: flex-start;
    -webkit-box-align: center;
    align-items: center;
    position: relative;
    min-width: 0px;
    box-sizing: border-box;
    text-align: left;
    padding-top: 8px;
    padding-bottom: 8px;
    padding-left: 16px;
    padding-right: 16px;
    height: 40px;
    outline: 0px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    margin: 0px 0px 4px;
    text-decoration: none;
    transition: background-color 150ms cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 4px;
}
.subitem-icon {
    margin-left: 4px;
    width: 10px;
    display: flex;
    -webkit-box-pack: center;
    justify-content: center;
}

.subitem.selected .sub-menu-spot {
    width: 8px;
    height: 8px;
    background-color: var(--main-color);
}

.subitem .sub-menu-spot {
    width: 4px;
    height: 4px;
    background-color: var(--menu-text-7-color);
    border-radius: 50%;
}
.subitem-text {
    flex: 1 1 auto;
    min-width: 0px;
    margin-top: 4px;
    margin-bottom: 4px;
    margin-left: 16px;
    color: var(--menu-text-7-color);
}
.subitem.selected .text-inner {
    font-weight: 700;
}

.subitem .sub-menu-spot {
    width: 4px;
    height: 4px;
    background-color: var(--menu-text-7-color);
    border-radius: 50%;
}
</style>

<script setup lang="ts">
export interface SubItem {
    title: string;
    to: string;
}
export interface MenuItem {
    icon?: string;
    title: string;
    to: string;
    subMenu?: SubItem[];
}
import { nextTick, onMounted, ref } from 'vue';
import SvgIcon from './SvgIcon.vue';
import { router } from '../constant';
const props = defineProps<{
    menuItems: MenuItem[];
    initRoutes?: boolean;
}>();
const menuItemRefs = ref<Record<number, HTMLDivElement>>({});
const subItemContainerRefs = ref<Record<number, HTMLDivElement>>({});
const subItemRefs = ref<Record<number, Record<number, HTMLDivElement>>>({});
const routeUpdate = ref<boolean>(false);
function handleItem(idx: number) {
    const elements = Object.entries(menuItemRefs.value);
    for (const [ei, element] of Object.entries(menuItemRefs.value)) {
        const i = +ei;
        if (i == idx) continue;
        element.classList.remove('selected');
        if (subItemRefs.value[+i as number] == null) {
            continue;
        }
        const subContainer = subItemContainerRefs.value[+i as number];
        if (subContainer == null) continue;
        subContainer.classList.remove('selected');
        subContainer.style.height = '0px';
    }
    (elements[idx]?.[1] as Element).classList.add('selected');

    const subContainer = subItemContainerRefs.value[idx];
    if (subContainer == null) {
        routerPush(props.menuItems[idx]?.to || '/');
        return;
    }

    if (!subContainer.classList.contains('selected')) {
        subContainer.style.height = 'auto';
        // get the height of the container
        const height = subContainer.getBoundingClientRect().height;
        // set the height to 0
        subContainer.style.height = '0px';
        // wait for the transition to end
        subContainer.getBoundingClientRect(); // refresh
        Object.values(subItemRefs.value[idx as number] || {}).forEach(
            (subElement) => {
                subElement.classList.remove('selected');
            },
        );
        nextTick(() => {
            // set the height to the actual height
            subContainer.style.height = `${height}px`;
            subContainer.classList.add('selected');
        });
        handleSubItem(idx, 0);
    }
}
function routerPush(path: string, subPath?: string) {
    if (!routeUpdate.value) return;
    const p = `/${path}`;
    if (subPath) {
        router.push(p + '/' + subPath);
    } else {
        router.push(p);
    }
}
function handleSubItem(idx: number, subIdx: number) {
    const elements = Object.values(subItemRefs.value[idx] || {});
    for (const element of elements) {
        element.classList.remove('selected');
    }
    const element = elements[subIdx] as Element;
    element.classList.add('selected');
    const path = props.menuItems[idx]?.to || '/';
    const subPath = props.menuItems[idx]?.subMenu?.[subIdx]?.to || '';
    routerPush(path, subPath);
}
onMounted(() => {
    routeUpdate.value = false;
    const initRoute: boolean = props.initRoutes || false;
    const route = router.currentRoute.value;
    const path = route.path.split('/')[1];
    const subPath = route.path.split('/')[2];
    const bidx = props.menuItems.findIndex((item) => item.to == path);
    const idx = initRoute && bidx == -1 ? 0 : bidx;
    if (!initRoute && bidx == -1) {
        routeUpdate.value = true;
        return;
    }
    routeUpdate.value = true;
    handleItem(idx);
    if (subPath) {
        const subIdx = props.menuItems[idx]?.subMenu?.findIndex(
            (item) => item.to == subPath,
        );
        if (subIdx != -1 && subIdx != undefined) {
            handleSubItem(idx, subIdx);
        }
    }
    routeUpdate.value = true;
});
</script>

<template>
    <Panel style="margin-bottom: 24px"
        ><div class="websites-overview">
            <div class="overview-side">
                共 {{ websites.length }} 个网站
                <InputEdit
                    class="search-input"
                    label="网站"
                    placeholder="支持网站名称、后端地址以及端口等搜索"
                ></InputEdit>
            </div>
            <div>
                <Button type="button" @click="toggleAddWebsite"
                    >添加网站</Button
                >
            </div>
        </div></Panel
    >
    <div class="websites">
        <Panel class="site" v-for="site in websites">
            <div class="site-overview">
                <div>
                    <SvgIcon
                        name="common-earth"
                        class="site-default-icon"
                        size="default"
                    ></SvgIcon>
                </div>
                <div class="site-view">
                    <PanelViewData class="small">
                        <template #title>今日请求</template>
                        <template #value>0</template>
                    </PanelViewData>
                    <div class="spt-line">
                        <div class="spt-line-inner"></div>
                    </div>
                    <PanelViewData class="small">
                        <template #title>今日流量</template>
                        <template #value>0b</template>
                    </PanelViewData>
                </div>
            </div>
            <div class="spt-line">
                <div class="spt-line-inner"></div>
            </div>
            <div class="site-content">
                <div>
                    {{ site.name || '无标题' }}
                </div>
                <div>
                    {{ site.hosts.join(', ') }}
                </div>
            </div>
        </Panel>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import Button from '../../components/Button.vue';
import InputEdit from '../../components/InputEdit.vue';
import Panel from '../../components/Panel.vue';
import type { Website } from '../../types/websites';
import { getWebsites } from '../../apis/websites';
import { addDialog } from '../../plugins/dialog';
import AddWebsite from '../../components/websites/AddWebsite.vue';
import SvgIcon from '../../components/SvgIcon.vue';
import PanelViewData from '../../components/PanelViewData.vue';
const websites = ref<Website[]>([]);

onMounted(async () => {
    websites.value = await getWebsites();
});

function toggleAddWebsite() {
    addDialog(AddWebsite);
}
</script>

<style>
:root {
    --site-spt-line: rgba(0, 0, 0, 0.3);
}
:root.dark {
    --site-spt-line: rgba(255, 255, 255, 0.3);
}
</style>

<style scoped>
.websites-overview {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1rem;
    min-height: 56px;
}

.overview-side {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1 1 auto;
}

.search-input {
    width: 288px;
    max-width: 100%;
}

.count {
    white-space: nowrap;
}

@media (max-width: 640px) {
    .overview-side {
        flex-wrap: wrap;
        width: 100%;
    }
    .search-input {
        width: 100%;
        max-width: 100%;
    }
    .websites-overview > :last-child {
        margin-left: 0;
        width: 100%;
    }
    .websites-overview > :last-child button {
        width: 100%;
    }
}

.websites {
    display: flex;
    flex-direction: row;
    min-height: calc(100% - 93px);
    margin-right: -10px;
    overflow: hidden;
    flex-wrap: wrap;
}

.site {
    display: flex;
    gap: 16px;
    flex: 1 1 auto;
}
.site-overview {
    display: flex;
    height: 100%;
    align-items: center;
    gap: 16px;
    flex-direction: column;
    flex-wrap: nowrap;
}
.site-default-icon {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    fill: var(--main-color);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
}
.spt-line {
    display: block;
    width: 1px;
    height: 100%;
    min-height: 100%;
}
.spt-line-inner {
    display: block;
    width: 1px;
    border-left: 0.5px solid var(--site-spt-line);
    height: 100%;
    min-height: 100%;
}
.site-view {
    height: 100%;
    display: flex;
    align-items: center;
    gap: 16px;
    flex: 1 1 auto;
}
.panel-view-data {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>

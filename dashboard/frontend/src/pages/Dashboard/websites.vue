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
        <Panel class="site" v-for="_ in websites">
            <div class="site-overview">
                <SvgIcon name="common-earth"></SvgIcon>
            </div>
            <div class="site-content"></div>
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
const websites = ref<Website[]>([]);

onMounted(async () => {
    websites.value = await getWebsites();
});

function toggleAddWebsite() {
    addDialog(AddWebsite);
}
</script>

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
}
</style>

<template>
    <Panel
        ><div class="websites-overview">
            <div class="overview-side">
                共 {{ 0 }} 个网站
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
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import Button from '../../components/Button.vue';
import InputEdit from '../../components/InputEdit.vue';
import Panel from '../../components/Panel.vue';
import type { Website } from '../../types';
import { getWebsites } from '../../api';
import { addDialog } from '../../plugins/dialog';
import AddWebsite from '../../components/websites/AddWebsite.vue';
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
</style>

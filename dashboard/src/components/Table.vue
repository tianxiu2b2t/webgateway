<template>
    <Panel style="padding: 0px">
        <div style="padding: 24px"><slot name="header"></slot></div>
        <table class="table">
            <thead class="thead">
                <tr class="tr">
                    <th
                        v-for="value in $props.config.headers"
                        class="th"
                        :style="{ width: value.width }"
                    >
                        {{ value.text }}
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr class="tr row" v-for="row in $props.data">
                    <td class="td" v-for="header in $props.config.headers">
                        {{ row[header.field] }}
                    </td>
                </tr>
            </tbody>
        </table>
        <div class="footer">
            <div class="ftcontainer">
                <InputEdit style="width: auto" />
                <span class="MuiBox-root css-okszq0"
                    >条每页, 共 {{ props.config.total }} 条</span
                >
                <div class="MuiStack-root css-1qftbaz" v-if="totalPages > 1">
                    <nav
                        aria-label="pagination navigation"
                        class="MuiPagination-root MuiPagination-text css-1xdhyk6"
                    >
                        <ul class="MuiPagination-ul css-51eq8m">
                            <li>
                                <button
                                    class="MuiButtonBase-root Mui-disabled MuiPaginationItem-root MuiPaginationItem-sizeMedium MuiPaginationItem-text MuiPaginationItem-circular MuiPaginationItem-colorPrimary MuiPaginationItem-textPrimary Mui-disabled MuiPaginationItem-previousNext css-caawcg"
                                    tabindex="-1"
                                    type="button"
                                    aria-label="Go to previous page"
                                    :disabled="!hasPrev"
                                    @click="currentPage--"
                                >
                                    <svg
                                        class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium MuiPaginationItem-icon css-4v85u4"
                                        focusable="false"
                                        aria-hidden="true"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"
                                        ></path>
                                    </svg>
                                </button>
                            </li>
                            <li v-for="val in displayPages">
                                <button
                                    class="MuiButtonBase-root MuiPaginationItem-root MuiPaginationItem-sizeMedium MuiPaginationItem-text MuiPaginationItem-circular MuiPaginationItem-colorPrimary MuiPaginationItem-textPrimary Mui-selected MuiPaginationItem-page css-caawcg"
                                    v-if="typeof val === 'number'"
                                    @click="currentPage = val"
                                >
                                    {{ val }}
                                </button>
                                <div
                                    v-else
                                    class="MuiPaginationItem-root MuiPaginationItem-sizeMedium MuiPaginationItem-text MuiPaginationItem-circular MuiPaginationItem-colorPrimary MuiPaginationItem-textPrimary MuiPaginationItem-ellipsis css-v1dd70"
                                >
                                    …
                                </div>
                            </li>
                            <li>
                                <button
                                    class="MuiButtonBase-root MuiPaginationItem-root MuiPaginationItem-sizeMedium MuiPaginationItem-text MuiPaginationItem-circular MuiPaginationItem-colorPrimary MuiPaginationItem-textPrimary MuiPaginationItem-previousNext css-caawcg"
                                    tabindex="0"
                                    type="button"
                                    aria-label="Go to next page"
                                    :disabled="!hasNext"
                                    @click="currentPage++"
                                >
                                    <svg
                                        class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium MuiPaginationItem-icon css-4v85u4"
                                        focusable="false"
                                        aria-hidden="true"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"
                                        ></path>
                                    </svg>
                                </button>
                            </li>
                        </ul>
                    </nav>
                    <div class="MuiStack-root css-14cb2qf">
                        <span class="pre-input-text">跳至</span>
                        <div
                            class="MuiFormControl-root MuiTextField-root css-1vzz7us"
                        >
                            <InputEdit />
                        </div>
                        <span class="aft-input-text"
                            >/ {{ totalPages }} 页</span
                        >
                    </div>
                </div>
            </div>
        </div>
    </Panel>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import InputEdit from './InputEdit.vue';
import Panel from './Panel.vue';
export interface TableHeaderConfig {
    text: string;
    width?: number | string;
    field: string;
}
export interface TableData {
    [key: string]: any;
}

export interface TableConfig {
    total: number;
    headers: TableHeaderConfig[];
}

const emit = defineEmits<{
    (e: 'currentPage', page: number): void;
    (e: 'pageSize', size: number): void;
}>();
const props = defineProps({
    config: {
        type: Object as () => TableConfig,
        required: true,
    },
    data: {
        type: Array as () => TableData[] | null,
        default: null,
    },
    currentPage: {
        type: Number,
        default: 1,
    },
    pageSize: {
        type: Number,
        default: 10,
    },
});
const totalPages = computed(() => {
    return Math.ceil(props.config.total / props.pageSize);
});
const displayPages = computed(() => {
    // first and last need,
    // current and prev and next
    // else use ...
    const pages = [];
    if (totalPages.value <= 7) {
        for (let i = 1; i <= totalPages.value; i++) {
            pages.push(i);
        }
    } else {
        if (currentPage.value <= 4) {
            for (let i = 1; i <= 5; i++) {
                pages.push(i);
            }
            pages.push('...');
            pages.push(totalPages.value);
        } else if (currentPage.value >= totalPages.value - 3) {
            pages.push(1);
            pages.push('...');
        }
    }
    return pages;
});
const currentPage = ref(props.currentPage);
const hasPrev = computed(() => {
    return currentPage.value > 1;
});
const hasNext = computed(() => {
    return currentPage.value < totalPages.value;
});
watch(
    () => currentPage.value,
    (newVal) => {
        console.log('newVal', newVal);
        console.log(hasPrev, hasNext);
        emit('currentPage', newVal);
    },
);
</script>
<style>
:root.dark {
    --table-border: var(--table-border);
    --table-th-text-color: rgba(255, 255, 255, 0.7);
    --table-tb-text-color: rgba(255, 255, 255);
    --table-tb-bg-color: rgb(35, 35, 35);
    --table-tb-bg-hover-color: rgba(255, 255, 255, 0.05);
}
:root {
    --table-border: rgb(243, 244, 245);
    --table-th-text-color: rgba(0, 0, 0, 0.7);
    --table-tb-text-color: rgba(0, 0, 0);
    --table-tb-bg-color: rgb(255, 255, 255);
    --table-tb-bg-hover-color: rgba(0, 0, 0, 0.05);
}
.table {
    display: table;
    width: 100%;
    border-spacing: 0px;
    border-collapse: separate;
    table-layout: fixed;
}
.thead {
    display: table-header-group;
}
.tr {
    color: inherit;
    display: table-row;
    vertical-align: middle;
    outline: 0px;
}
.th:first-of-type {
    padding-left: 24px;
}
.th {
    font-family: inherit;
    display: table-cell;
    vertical-align: inherit;
    border-bottom: 1px solid var(--table-border);
    padding: 0px 16px 0px 0px;
    font-weight: 500;
    text-align: left;
    position: sticky;
    top: 0px;
    z-index: 2;
    background: var(--table-border);
    line-height: 1.5;
    border-top-color: var(--table-border);
    border-right-color: var(--table-border);
    border-left-color: var(--table-border);
    color: var(--table-th-text-color);
    font-size: 12px;
    height: 24px;
}
.td:first-of-type {
    padding-left: 24px;
}
.td {
    font-family: inherit;
    font-weight: 400;
    display: table-cell;
    vertical-align: inherit;
    border-bottom: 1px solid var(--table-border);
    text-align: left;
    padding: 24px 16px 24px 0px;
    color: var(--table-tb-text-color);
    background: var(--table-tb-bg-color);
    line-height: 1.5;
    font-size: 14px;
    border-top-color: var(--table-border);
    border-right-color: var(--table-border);
    border-left-color: var(--table-border);
}
.row:hover .td {
    background-color: var(--table-tb-bg-hover-color);
}
.footer {
    padding: 24px;
}
.ftcontainer {
    display: flex;
    flex-direction: row;
    -webkit-box-align: center;
    align-items: center;
    font-size: 14px;
}
.css-okszq0 {
    margin-left: 12px;
    color: rgba(255, 255, 255, 0.5);
    margin-right: auto;
}
.css-1qftbaz {
    display: flex;
    flex-direction: row;
    -webkit-box-align: center;
    align-items: center;
}
.css-51eq8m {
    display: flex;
    flex-wrap: wrap;
    -webkit-box-align: center;
    align-items: center;
    padding: 0px;
    margin: 0px;
    list-style: none;
}
.css-caawcg[disabled] {
    opacity: 0.38;
}
.css-caawcg[disabled] {
    pointer-events: none;
    cursor: default;
}
.css-caawcg {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    font-family: inherit;
    font-weight: 400;
    font-size: 0.875rem;
    line-height: 1.43;
    text-align: center;
    box-sizing: border-box;
    min-width: 32px;
    height: 32px;
    color: rgb(255, 255, 255);
    outline: 0px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    text-decoration: none;
    border-radius: 16px;
    padding: 0px 6px;
    margin: 0px 3px;
    transition:
        color 250ms cubic-bezier(0.4, 0, 0.2, 1),
        background-color 250ms cubic-bezier(0.4, 0, 0.2, 1);
}
.css-14cb2qf {
    display: flex;
    flex-direction: row;
    -webkit-box-align: center;
    align-items: center;
    color: rgba(255, 255, 255, 0.5);
}
.css-1vzz7us {
    display: inline-flex;
    flex-direction: column;
    position: relative;
    min-width: 0px;
    padding: 0px;
    border: 0px;
    vertical-align: top;
    width: 59px;
    margin: 0px 8px;
}
.css-4v85u4 {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 1.25rem;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
    margin: 0px -8px;
}
</style>

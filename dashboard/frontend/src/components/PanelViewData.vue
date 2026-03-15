<template>
    <div class="title">
        <div class="title-text"><slot name="title"></slot></div>
    </div>
    <div class="value">
        <div class="">
            <slot name="value"></slot>
        </div>
    </div>
</template>

<script setup lang="ts"></script>
<style>
:root {
    --panel-view-data-title-color: rgba(0, 0, 0, 0.5);
    --panel-view-data-value-color: rgb(0, 0, 0);
}
:root.dark {
    --panel-view-data-title-color: rgba(255, 255, 255, 0.5);
    --panel-view-data-value-color: rgb(255, 255, 255);
}
</style>
<style scoped>
.title {
    display: flex;
    flex-direction: row;
    -webkit-box-align: center;
    place-items: center;
    color: var(--panel-view-data-title-color);
    font-size: 12px;
    margin-bottom: 6px;
}
.title-text {
    margin-right: 8px;
    -webkit-box-flex: 0;
    flex-grow: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-size: 14px;
}
.value {
    font-size: 20px;
    font-weight: 650;
    color: var(--panel-view-data-value-color);
}
</style>

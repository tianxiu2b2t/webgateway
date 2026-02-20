<template>
    <div class="dialog-root" @click="click">
        <div
            class="dialog-backdrop"
            style="
                opacity: 1;
                transition: opacity 225ms cubic-bezier(0.4, 0, 0.2, 1);
            "
        ></div>
        <div class="dialog-container">
            <Panel class="panel">
                <div class="dialog-header"><slot name="header"></slot></div>
                <div class="dialog-content"><slot name="content"></slot></div>
                <div class="dialog-footer"><slot name="footer"></slot></div
            ></Panel>
        </div>
    </div>
</template>

<script lang="ts" setup>
import Panel from '../../components/Panel.vue';

function click(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (target.closest('.dialog-container')) {
        emit('close');
    }
}
</script>

<style>
:root {
    --dialog-panel-shadow:
        0px 11px 15px -7px rgba(0, 0, 0, 0.2),
        0px 24px 38px 3px rgba(0, 0, 0, 0.12),
        0px 9px 46px 8px rgba(0, 0, 0, 0.12);
}
</style>
<style scoped>
.dialog-root {
    position: fixed;
    z-index: 1300;
    inset: 0px;
}
.dialog-backdrop {
    position: fixed;
    display: flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    inset: 0px;
    background-color: rgba(0, 0, 0, 0.8);
    -webkit-tap-highlight-color: transparent;
    z-index: -1;
}
.dialog-container {
    height: 100%;
    outline: 0px;
    display: flex;
    -webkit-box-pack: center;
    justify-content: center;
    -webkit-box-align: center;
    align-items: center;
}
.panel {
    min-width: unset;
    margin: 32px;
    max-height: calc(100% - 64px);
    max-width: 600px;
    box-shadow: var(--dialog-panel-shadow);
    padding: 0;
}
.dialog-header {
    margin: 0px;
    font-family: inherit;
    line-height: 1.6;
    flex: 0 0 auto;
    font-weight: 600;
    font-size: 16px;
    padding: 32px 24px 16px;
    display: flex;
    -webkit-box-pack: justify;
    justify-content: space-between;
    -webkit-box-align: center;
    align-items: center;
}
.dialog-header button {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    text-align: center;
    font-size: 1.5rem;
    --IconButton-hoverBg: rgba(0, 0, 0, 0.04);
    color: rgba(0, 0, 0, 0.5);
    outline: 0px;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    margin: 0px;
    text-decoration: none;
    flex: 0 0 auto;
    padding: 8px;
    border-radius: 50%;
    transition: background-color 150ms cubic-bezier(0.4, 0, 0.2, 1);
}
.dialog-header .icon {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    font-size: 20px;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
.dialog-content {
    flex: 1 1 auto;
    overflow-y: auto;
    padding: 20px 24px;
}
.dialog-footer + .dialog-content {
    padding-bottom: 0px;
}
.dialog-footer {
    margin: 0px;
    font-family: inherit;
    line-height: 1.6;
    flex: 0 0 auto;
    font-weight: 600;
    font-size: 16px;
    padding: 8px;
    display: flex;
    -webkit-box-pack: justify;
    justify-content: space-between;
    -webkit-box-align: center;
    align-items: center;
}
.dialog-header + .dialog-content {
    padding-top: 0px;
}
</style>

<template>
    <div class="panel"><slot></slot></div>
</template>

<style lang="css">
:root {
    --panel-shadow:
        0px 12px 24px -4px rgba(0, 4, 10, 0.06),
        0px 0px 2px 0px rgba(0, 4, 10, 0.06);
    --panel-bg-color: rgb(255, 255, 255);
}
:root.dark {
    --panel-bg-color: rgb(35, 35, 35);
    --panel-shadow:
        0px 12px 24px -4px rgba(0, 4, 10, 0.06),
        0px 0px 2px 0px rgba(0, 4, 10, 0.06);
    --panel-overlay: linear-gradient(
        rgba(255, 255, 255, 0.051),
        rgba(255, 255, 255, 0.051)
    );
    --panel-text-color: rgb(255, 255, 255);
}
</style>
<style lang="css" scoped>
.panel {
    background-color: var(--panel-bg-color);
    color: var(--panel-text-color);
    box-shadow: var(--panel-shadow);
    background-image: none;
    display: flex;
    flex-direction: column;
    -webkit-box-align: center;
    align-items: center;
    width: 440px;
    transition: box-shadow 300ms cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 8px;
    padding: 30px 48px;
}
</style>

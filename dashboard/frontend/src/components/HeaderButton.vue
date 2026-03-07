<script setup lang="ts"></script>

<template>
    <button class="hbtn" tabindex="0" type="button">
        <div class="icon">
            <slot></slot>
        </div>
    </button>
</template>

<style>
.hbtn:hover {
    box-shadow: none;
    color: var(--main-color);
    font-weight: 700;
}

:root.dark {
    --variant-containedColor: #f1d6bf;
    --variant-containedBg: #232323;
}
:root {
    --variant-containedColor: rgba(0, 0, 0, 0.7);
    --variant-containedBg: #fff;
}
.hbtn {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    position: relative;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    font-family: inherit;
    font-weight: 500;
    line-height: 1.75;
    text-transform: uppercase;
    color: var(--variant-containedColor);
    background-color: var(--variant-containedBg);
    font-size: 0.8125rem;
    box-shadow: none;
    min-width: 36px;
    height: 36px;
    outline: 0px;
    text-decoration: none;
    border-width: 0px;
    border-style: initial;
    border-color: initial;
    border-image: initial;
    transition:
        background-color 250ms cubic-bezier(0.4, 0, 0.2, 1),
        box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1),
        border-color 250ms cubic-bezier(0.4, 0, 0.2, 1);
    padding: 0px;
    border-radius: 50%;
}
.icon {
    width: 1.25em;
    height: 1.25em;
    fill: currentcolor;
    font-size: 16px;
}
</style>

<template>
    <Dialog>
        <template #header
            ><div class="MuiBox-root css-wazrjc">
                <SvgIcon
                    name="presentation-alert"
                    class="css-rdjmgo"
                    size="1em"
                ></SvgIcon
                >请确认是否关闭
            </div></template
        >
        <template #content>
            <span class="content">有未保存的内容，是否关闭？</span>
        </template>
        <template #footer
            ><DialogClose @cancel="$emit('cancel')" @confirm="$emit('confirm')"
        /></template>
    </Dialog>
</template>

<script setup lang="ts">
import SvgIcon from '../../../components/SvgIcon.vue';
import Dialog from '../Dialog.vue';
import DialogClose from '../DialogClose.vue';
const emit = defineEmits(['cancel', 'confirm']);
</script>

<style lang="css" scoped>
.css-wazrjc {
    display: flex;
    -webkit-box-align: center;
    align-items: center;
    line-height: 22px;
    font-weight: 500;
}
.css-rdjmgo {
    user-select: none;
    width: 1em;
    height: 1em;
    display: inline-block;
    flex-shrink: 0;
    fill: currentcolor;
    color: rgb(255, 191, 0);
    margin-right: 16px;
    font-size: 24px;
    transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
.content {
    padding-left: 40px;
}
</style>

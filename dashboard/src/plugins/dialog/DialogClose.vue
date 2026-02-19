<script setup lang="ts">
import Button from '../../components/Button.vue';
</script>

<template>
    <div class="MuiStack-root css-1j66doz">
        <div class="MuiDialogActions-root MuiDialogActions-spacing css-1vwkjbq">
            <Button :reverse-color="true">取消</Button>
            <!-- <button
                class="MuiButtonBase-root MuiButton-root MuiButton-text MuiButton-textPrimary MuiButton-sizeMedium MuiButton-textSizeMedium MuiButton-colorPrimary MuiButton-root MuiButton-text MuiButton-textPrimary MuiButton-sizeMedium MuiButton-textSizeMedium MuiButton-colorPrimary css-w4kzji"
                tabindex="0"
                type="button"
            >
                取消</button
            >--><Button>确定</Button>
        </div>
    </div>
</template>

<style>
.css-1j66doz {
    display: flex;
    flex-direction: row-reverse;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: justify;
    justify-content: space-between;
    width: 100%;
}
.css-1vwkjbq {
    display: flex;
    -webkit-box-align: center;
    align-items: center;
    padding: 8px;
    -webkit-box-pack: end;
    justify-content: flex-end;
    flex: 0 0 auto;
}
</style>

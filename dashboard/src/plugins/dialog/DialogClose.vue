<script setup lang="ts">
import Button from '../../components/Button.vue';
</script>

<template>
    <div class="dialog-close-root">
        <div class="dialog-close-container">
            <Button :reverse-color="true" @click="$emit('cancel')">取消</Button
            ><Button @click="$emit('confirm')">确定</Button>
        </div>
    </div>
</template>

<style>
.dialog-close-root {
    display: flex;
    flex-direction: row-reverse;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: justify;
    justify-content: space-between;
    width: 100%;
}
.dialog-close-container {
    display: flex;
    -webkit-box-align: center;
    align-items: center;
    padding: 8px;
    -webkit-box-pack: end;
    justify-content: flex-end;
    flex: 0 0 auto;
    gap: 16px;
}
</style>

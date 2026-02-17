<template>
    <div class="auth-container">
        <div class="auth-inner-container">
            <Panel>
                <h4 class="title">
                    <span>登陆</span>
                    <p class="app-name">Web Gateway</p>
                </h4>
                <form @submit.prevent="handle">
                    <div class="inputarea">
                        <InputEdit
                            label="用户名"
                            v-model:value="username"
                            @keydown.enter="handle"
                            ref="usernameRef"
                        ></InputEdit>
                        <InputEdit
                            label="一次性密码"
                            v-model:value="totp"
                            @keydown.enter="handle"
                            ref="totpRef"
                        ></InputEdit>
                    </div>
                    <Button
                        text="登陆"
                        type="submit"
                        :click="handle"
                        :processing="processing"
                        >登陆</Button
                    >
                </form>
            </Panel>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import Button from '../components/Button.vue';
import InputEdit from '../components/InputEdit.vue';
import Panel from '../components/Panel.vue';
import { login } from '../auth';
import { router } from '../constant';
const username = ref('');
const totp = ref('');
const processing = ref(false);
const usernameRef = ref();
const totpRef = ref();

async function handle() {
    if (username.value.trim() == '') {
        console.log(usernameRef.value?.focus);
        usernameRef.value.focus();
        return;
    }
    if (totp.value.trim() == '') {
        totpRef.value?.focus();
        return;
    }
    processing.value = true;
    try {
        const result = await login(username.value, totp.value);
        if (result) {
            router.push('/');
        }
    } finally {
        processing.value = false;
    }
}
</script>

<style scoped>
.inputedit-container {
    margin-top: 2rem;
}
.auth-container {
    height: 100vh;
    width: 100vw;
    z-index: 1000;
    position: fixed;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
}
.auth-inner-container {
    width: 400px;
    height: 400px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}
.title {
    display: flex;
    width: 100%;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
}
.app-name {
    color: var(--text-color);
}
</style>

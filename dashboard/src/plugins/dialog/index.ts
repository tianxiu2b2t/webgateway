import { computed, ref, type Component } from 'vue';
import Dialog from './Dialog.vue';

const inner = ref<Component[]>([]);

export function addDialog(dialog: Component) {
    inner.value.push(dialog);
}

export const dialogs = computed(() => {
    return inner.value;
});

import { computed, ref, type Component } from 'vue';
import Dialog from './Dialog.vue';

const inner = ref<Component<typeof Dialog>[]>([]);

export function addDialog(dialog: Component<typeof Dialog>) {
    inner.value.push(dialog);
}

export const dialogs = computed(() => {
    return inner.value;
});

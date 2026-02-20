import { computed, ref, type Component } from 'vue';

const inner = ref<DialogComponent[]>([]);
let dialogId = 0;

export interface DialogComponent {
    component: Component;
    id: number;
    out: boolean;
}

export function addDialog(dialog: Component) {
    inner.value.push({
        component: dialog,
        id: dialogId++,
        out: false,
    });
}

export function removeDialog(id: number) {
    const component = inner.value.find((dialog) => dialog.id === id);
    if (!component) {
        return;
    }
    component.out = true;
    setTimeout(() => {
        inner.value = inner.value.filter((dialog) => dialog.id !== id);
    }, 225);
}

export const dialogs = computed(() => {
    return inner.value;
});

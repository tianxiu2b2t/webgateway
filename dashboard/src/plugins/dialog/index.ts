import { computed, markRaw, ref, type Component } from 'vue';

const inner = ref<DialogComponent[]>([]);
let dialogId = 0;

const defaultOptions: DialogOptions = {
    preventCancel: false,
};

export interface DialogOptions {
    preventCancel?: boolean;
}

export interface DialogComponent {
    component: Component;
    id: number;
    out: boolean;
    options: DialogOptions;
}

export function addDialog(dialog: Component, options?: DialogOptions) {
    inner.value.push({
        component: markRaw(dialog),
        id: dialogId++,
        out: false,
        options: {
            ...defaultOptions,
            ...options,
        },
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

export function removeDialogFromCancel(id: number) {
    console.log(id);
    const component = inner.value.find((dialog) => dialog.id === id);
    if (!component) {
        return;
    }

    if (component.options.preventCancel) {
        return;
    }

    removeDialog(id);
}

export const dialogs = computed(() => {
    return inner.value;
});

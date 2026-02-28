import { computed, markRaw, ref, type Component } from 'vue';

const inner = ref<DialogComponent[]>([]);
let dialogId = 0;

const defaultOptions: DialogOptions = {
    preventCancel: false,
    preventConfirm: true,
};

export interface DialogOptions {
    preventCancel?: boolean;
    preventConfirm?: boolean;
}

export interface DialogComponent {
    component: Component;
    id: number;
    out: boolean;
    options: DialogOptions;
    props?: Record<string, any>;
}

export function addDialog(
    dialog: Component,
    props?: Record<string, any>,
    options?: DialogOptions,
): number {
    let id = dialogId++;
    inner.value.push({
        component: markRaw(dialog),
        id,
        out: false,
        options: {
            ...defaultOptions,
            ...options,
        },
        props,
    });
    return id;
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
    const component = inner.value.find((dialog) => dialog.id === id);
    if (!component) {
        return;
    }

    if (component.options.preventCancel) {
        return;
    }

    removeDialog(id);
}

export function removeDialogFromConfirm(id: number) {
    const component = inner.value.find((dialog) => dialog.id === id);
    if (!component) {
        return;
    }

    if (component.options.preventConfirm) {
        return;
    }

    removeDialog(id);
}

export const dialogs = computed(() => {
    return inner.value;
});

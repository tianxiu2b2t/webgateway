import { computed, markRaw, ref, type Component } from 'vue';

const inner = ref<DialogComponent[]>([]);
let dialogId = 0;
const listeners: Record<number, Function> = {};
let listenerId = 0;

const defaultOptions: DialogOptions = {
    preventCancel: false,
    preventConfirm: false,
};

export type DialogEventType = 'close' | 'confirm' | 'cancel' | 'open';

class DialogEvent extends Event {
    public type: DialogEventType;
    public component: Component;
    id: number;
    constructor(type: DialogEventType, component: Component, id: number) {
        super('dialog');
        this.type = type;
        this.component = component;
        this.id = id;
    }
}

export class DialogCloseEvent extends DialogEvent {
    constructor(component: Component, id: number) {
        super('close', component, id);
    }
}

export class DialogCancelEvent extends DialogEvent {
    constructor(component: Component, id: number) {
        super('cancel', component, id);
    }
}

export class DialogConfirmEvent extends DialogEvent {
    constructor(component: Component, id: number) {
        super('confirm', component, id);
    }
}

export class DialogOpenEvent extends DialogEvent {
    constructor(component: Component, id: number) {
        super('open', component, id);
    }
}

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
    window.dispatchEvent(new DialogOpenEvent(dialog, id));

    return id;
}

export function removeDialog(id: number) {
    const component = inner.value.find((dialog) => dialog.id === id);
    if (!component) {
        return;
    }
    window.dispatchEvent(new DialogCloseEvent(component.component, id));

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

    window.dispatchEvent(new DialogCancelEvent(component.component, id));

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

    window.dispatchEvent(new DialogConfirmEvent(component.component, id));

    removeDialog(id);
}

export const dialogs = computed(() => {
    return inner.value;
});

export function listen(
    type: DialogEventType,
    callback: (event: DialogEvent) => void,
    id?: number,
): number {
    const listenedId = ++listenerId;
    listeners[listenedId] = (event: DialogEvent) => {
        const dialogEvent = event;
        if (dialogEvent.type === type && (!id || dialogEvent.id === id)) {
            callback(dialogEvent);
        }
    };
    window.addEventListener('dialog', listeners[listenedId] as EventListener);
    return listenedId;
}

export function unlisten(id: number) {
    window.removeEventListener('dialog', listeners[id] as EventListener);
}

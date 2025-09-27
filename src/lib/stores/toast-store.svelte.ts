type ToastType = "info" | "yayy" | "warn" | "uhoh";

type Toast = {
    message: string;
    id: string;
    type: ToastType;
    duration: number;
}

const toasts = $state<Toast[]>([]);

export const toastStore = {
    get toasts() { return toasts; },
    show(message: string, type: ToastType, duration: number = 5000) {
        const id = crypto.randomUUID();
        toasts.unshift({ id, message, type, duration });
        setTimeout(() => this.remove(id), duration);
    },
    remove(id: string) {
        const i = toasts.findIndex(t => t.id === id);
        if (i >= 0) {
            toasts.splice(i, 1);
        }
    },
}

import { writable } from "svelte/store";

type ToastSeverity = "success" | "error";
type Toast = {
    message: string;
    severity: ToastSeverity;
};

export let toasts = writable(new Map<string, Toast>());

export function deleteToast(id: string) {
    toasts.update(m => {
        m.delete(id);
        return m;
    });
}

export function pushToast(
    message: string,
    severity: ToastSeverity,
    delay: number = 5000
) {
    const id = crypto.randomUUID().toString();
    toasts.update(m => m.set(id, { message, severity }));
    setTimeout(() => deleteToast(id), delay);
}

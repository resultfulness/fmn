import { PUBLIC_API_URL } from "$env/static/public";
import { toastStore } from "$lib/stores/toast-store.svelte";
import { Errors } from "$lib/types/error";
import { fetch } from "@tauri-apps/plugin-http";

export async function apiFetch(
    endpoint: string,
    method?: "GET" | "POST" | "PATCH" | "PUT" | "DELETE",
    body?: object
): Promise<Response> {
    try {
        const res = await fetch(`${PUBLIC_API_URL}${endpoint}`, {
            method: method || "GET",
            body: JSON.stringify(body) || undefined,
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            danger: {
                acceptInvalidCerts: true,
                acceptInvalidHostnames: false,
            }
        });
        toastStore.clearInfinite();
        return res;
    } catch (e) {
        toastStore.setInfinite(Errors.ApiAccessError.message, "uhoh");
        throw Errors.ApiAccessError;
    }
}

import { PUBLIC_API_URL } from "$env/static/public";
import { pushToast } from "$lib/ui/toast";

const API_URL = PUBLIC_API_URL;

async function apiFetch(endpoint: string, options: RequestInit = {}) {
    const url = `${API_URL}${endpoint}`;

    const headers: HeadersInit = { "Content-Type": "application/json" };

    const res = await request
        .fetch(url, {
            ...options,
            headers: {
                ...headers,
                ...options.headers,
            },
        })
        .catch(e => {
            pushToast("couldn't connect to api", "error");
            throw e;
        });

    const data = await res.json();
    if (!res.ok) {
        pushToast(data.error, "error");
    }
    return data;
}

const request = {
    fetch: window.fetch,
    async get(endpoint: string) {
        return await apiFetch(endpoint);
    },
    async post(endpoint: string, data: any) {
        return await apiFetch(endpoint, {
            method: "POST",
            body: JSON.stringify(data),
        });
    },
    async put(endpoint: string, data: any) {
        return await apiFetch(endpoint, {
            method: "PUT",
            body: JSON.stringify(data),
        });
    },
    async patch(endpoint: string, data: any) {
        return await apiFetch(endpoint, {
            method: "PATCH",
            body: JSON.stringify(data),
        });
    },
    async delete(endpoint: string) {
        return await apiFetch(endpoint, { method: "DELETE" });
    },
};

export default request;

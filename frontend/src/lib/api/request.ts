import { PUBLIC_API_URL } from "$env/static/public";

const API_URL = PUBLIC_API_URL;

export class APIError extends Error {
    public status: number;

    constructor(message: string, status: number) {
        super(message);
        this.status = status;
    }
}

async function apiFetch(endpoint: string, options: RequestInit = {}) {
    const url = `${API_URL}${endpoint}`;

    const headers: HeadersInit = {
        "Content-Type": "application/json",
    };

    return fetch(url, {
        ...options,
        headers: {
            ...headers,
            ...options.headers,
        },
    })
        .catch(() => {
            throw new Error("couldn't connect to api");
        })
        .then(async res => {
            const data = await res.json();
            if (!res.ok) {
                throw new APIError(data.error, res.status);
            }
            return data;
        });
}

const request = {
    async get(endpoint: string) {
        return apiFetch(endpoint);
    },
    async post(endpoint: string, data: any) {
        return apiFetch(endpoint, {
            method: "POST",
            body: JSON.stringify(data),
        });
    },
    async patch(endpoint: string, data: any) {
        return apiFetch(endpoint, {
            method: "PATCH",
            body: JSON.stringify(data),
        });
    },
    async delete(endpoint: string) {
        return apiFetch(endpoint, { method: "DELETE" });
    },
};

export default request;

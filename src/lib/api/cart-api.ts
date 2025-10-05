import { PUBLIC_API_URL } from "$env/static/public";
import type { CartItem } from "$lib/types";
import { fetch } from "@tauri-apps/plugin-http";

const cartApi = {
    // eventSource: new EventSource(`${PUBLIC_API_URL}/cart/stream`),
    // stream(callback: () => void) {
    //     this.eventSource.addEventListener("cart", e => {
    //         console.log(e);
    //     })
    // },
    subscribers: new Map(),
    subscribe(callback: (m: CartItem[]) => void) {
        const key = Math.max.apply(this.subscribers.keys()) + 1
        this.subscribers.set(key, callback);
        return key
    },
    unsubscribe(key: number) {
        this.subscribers.delete(key);
    },
    async openStream() {
        const response = await fetch(`${PUBLIC_API_URL}/cart/stream`, {
            headers: { "Accept": "text/event-stream" }
        });

        const reader = response.body?.getReader();
        const decoder = new TextDecoder();
        let buffer = ''

        if (!reader) {
            return;
        }

        while (true) {
            const { done, value } = await reader.read();
            if (done) break;

            const chunk = decoder.decode(value, { stream: true });
            buffer += chunk;
            while (buffer.indexOf('\n\n') !== -1) {
                let message = buffer.split('\n\n')[0];
                // Handle the message
                const messageLines = message.split("\n");
                const eventName = messageLines[0].replace("event: ", "");
                const eventData = JSON.parse(messageLines[1].replace("data: ", ""));
                if (eventName === "cart")
                    this.subscribers.forEach(s => s(eventData));
                buffer = buffer.replace(`${message}\n\n`, "");
            }
        }
    }
};

export default cartApi;

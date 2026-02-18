<script module lang="ts">
import { SvelteMap } from "svelte/reactivity";

type ToastSeverity = "success" | "error";
type Toast = {
    message: string;
    severity: ToastSeverity;
};

let toasts: SvelteMap<number, Toast> = $state(new SvelteMap());

export function pushToast(
    message: string,
    severity: ToastSeverity,
    delay: number = 5000
) {
    const timestamp = +new Date();
    toasts.set(timestamp, { message, severity });
    setTimeout(() => toasts.delete(timestamp), delay);
}
</script>

<script>
import { Check, CircleAlert } from "@lucide/svelte";
</script>

<div class="toasts">
    {#each toasts as [t, { message, severity }]}
        <button class={`toast ${severity}`} onclick={() => toasts.delete(t)}>
            {#if severity === "success"}
                <Check />
            {:else}
                <CircleAlert />
            {/if}
            {message}
        </button>
    {/each}
</div>

<style>
.toasts {
    position: fixed;
    top: 0.75rem;
    right: 0.75rem;
    display: grid;
    justify-items: end;
    gap: 0.5rem;
    z-index: 9999;
}

.toast {
    background-color: var(--clr-surface);
    color: var(--clr-text);
    padding: 0.5rem 0.75rem;
    border-radius: var(--rounding);
    box-shadow: var(--shadow);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border: none;
}

.toast.success {
    background-color: var(--clr-success);
    color: var(--clr-dark);
}

.toast.error {
    background-color: var(--clr-danger);
    color: var(--clr-dark);
}
</style>

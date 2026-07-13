<script>
import { Check, CircleAlert } from "@lucide/svelte";
import { deleteToast, toasts } from "./toast";
</script>

<div class="toasts">
    {#each $toasts as [t, { message, severity }]}
        <button class={`toast ${severity}`} onclick={() => deleteToast(t)}>
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
    margin-left: 0.75rem;
    display: flex;
    flex-direction: column-reverse;
    align-items: end;
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

    &.success {
        background-color: var(--clr-success);
        color: var(--clr-dark);
    }

    &.error {
        background-color: var(--clr-danger);
        color: var(--clr-dark);
    }
}
</style>

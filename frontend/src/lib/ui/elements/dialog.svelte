<script lang="ts">
import { X } from "@lucide/svelte";
import type { Snippet } from "svelte";
import IconButton from "../molecules/icon-button.svelte";

interface DialogProps {
    dialog: HTMLDialogElement;
    onclose?: () => void;
    children?: Snippet;
    title: string;
}

let {
    dialog = $bindable(),
    onclose = () => {},
    children,
    title,
}: DialogProps = $props();
</script>

<dialog bind:this={dialog} {onclose} closedby="any">
    <header>
        <h2 class="text-heading">{title}</h2>
        <IconButton
            variant="secondary"
            icon={X}
            onclick={() => dialog.close()}
        />
    </header>
    {@render children?.()}
</dialog>

<style>
dialog {
    border: 0;
    border-radius: var(--rounding);
    background-color: var(--clr-base);
    color: var(--clr-text);
    box-shadow: var(--shadow);
    padding: 1rem;
    gap: 1rem;

    width: min(100% - 4rem, 320px);

    &[open] {
        display: grid;
    }

    &::backdrop {
        background-color: rgba(0, 0, 0, 0.5);
    }
}

header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    word-break: break-word;
}

h2 {
    margin: 0;
}
</style>

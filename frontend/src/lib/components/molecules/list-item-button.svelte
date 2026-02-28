<script lang="ts">
import type { Snippet } from "svelte";
import type { MouseEventHandler } from "svelte/elements";

interface ListItemButtonProps {
    children: Snippet;
    onclick?: MouseEventHandler<HTMLButtonElement> | null;
    ariaLabel?: string;
}

const { children, onclick, ariaLabel }: ListItemButtonProps = $props();
</script>

<li>
    <button {onclick} aria-label={ariaLabel} class="text-content">
        {@render children()}
    </button>
</li>

<style>
button {
    background-color: var(--clr-surface);
    border-radius: var(--rounding);
    padding: 0.5rem;
    border: none;
    width: 100%;
    text-align: left;
    display: block;

    &:focus-visible {
        outline: 2px solid var(--clr-primary);
    }

    &:hover {
        cursor: pointer;
    }
}
</style>

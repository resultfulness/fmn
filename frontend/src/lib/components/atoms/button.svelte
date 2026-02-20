<script lang="ts">
import type { Snippet } from "svelte";
import type { MouseEventHandler } from "svelte/elements";

export interface ButtonProps {
    onclick?: MouseEventHandler<HTMLButtonElement> | null;
    children: Snippet;
    type?: "button" | "submit" | "reset";
    variant?: "primary" | "secondary" | "danger";
    disabled?: boolean;
    ariaLabel?: string;
    href?: string;
}

let {
    onclick = () => {},
    children,
    type = "submit",
    variant = "primary",
    disabled,
    href,
    ariaLabel,
}: ButtonProps = $props();
</script>

{#if href}
    <a
        {href}
        aria-label={ariaLabel}
        class="button"
        class:primary={variant === "primary"}
        class:secondary={variant === "secondary"}
        class:danger={variant === "danger"}
    >
        {@render children()}
    </a>
{:else}
    <button
        {onclick}
      {type}
        {disabled}
        aria-label={ariaLabel}
        class="button text-emph"
        class:primary={variant === "primary"}
        class:secondary={variant === "secondary"}
        class:danger={variant === "danger"}
    >
        {@render children()}
    </button>
{/if}

<style>
.button {
    position: relative;
    padding: 0.75rem 1rem;
    border-radius: var(--rounding);
    box-shadow: var(--shadow);
    border: 0;
    cursor: pointer;
    transition: background-color 200ms;
    display: grid;
    place-items: center;
    text-decoration: none;
}

.button.primary {
    background-color: var(--clr-primary);
    color: var(--clr-dark);
}

.button.primary:hover {
    background-color: var(--clr-primary-shade);
}

.button:focus-visible {
    outline: 2px solid var(--clr-primary);
    outline-offset: 2px;
}

.button.secondary {
    background-color: var(--clr-surface);
    color: var(--clr-text);
}

.button.secondary:hover {
}

.button:disabled {
    background-color: var(--clr-text-dim);
    color: var(--clr-text);
}

.button.danger {
    background-color: var(--clr-danger);
    color: var(--clr-dark);
}
</style>

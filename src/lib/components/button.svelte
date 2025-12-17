<script lang="ts">
import type { Snippet } from "svelte";
import type { MouseEventHandler } from "svelte/elements";

interface ButtonProps {
    onclick?: MouseEventHandler<HTMLButtonElement> | null
    children: Snippet;
    type?: "button" | "submit" | "reset";
    variant?: "primary" | "secondary" | "alert" | "empty";
    disabled?: boolean;
    icon?: boolean;
    fab?: boolean;
    ariaLabel?: string;
    href?: string;
}

let {
    onclick = () => {},
    children,
    type = "submit",
    variant = "primary",
    disabled,
    icon,
    fab,
    href,
    ariaLabel,
}: ButtonProps = $props();
</script>

<button
    {onclick}
    {type}
    {disabled}
    aria-label={ariaLabel}
    class="button"
    class:button--primary={variant === "primary"}
    class:button--secondary={variant === "secondary"}
    class:button--alert={variant === "alert"}
    class:button--empty={variant === "empty"}
    class:button--icon={icon}
    class:button--fab={fab}
>
    {#if href}
        <a
            {href}
            aria-label={ariaLabel}
            style="position: absolute; inset: 0;"
        ></a>
    {/if}
    {@render children()}
</button>

<style>
.button {
    position: relative;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    border: 0;
    cursor: pointer;
    transition: background-color 200ms;
    font: inherit;
    font-weight: 500;
    display: grid;
    place-items: center;
}

.button--primary {
    background-color: var(--clr-primary);
    color: var(--clr-dark);
}

.button:focus {
    outline: 2px solid var(--clr-primary);
    outline-offset: 2px;
}

.button--primary:hover {
    background-color: var(--clr-primary-shade);
}

.button--secondary {
    background-color: var(--clr-s1);
    color: var(--clr-text);
}

.button--secondary:hover {
    background-color: var(--clr-s2);
}

.button--empty {
    background-color: transparent;
    color: var(--clr-text);
    padding: 0;
}

.button--icon {
    padding: 0.5rem;
}

.button:disabled {
    background-color: var(--clr-text-mute);
    color: var(--clr-s1);
}

.button--alert {
    background-color: var(--clr-error);
    color: var(--clr-base);
}

.button--fab {
    position: fixed;
    padding: 0.5rem;
    bottom: 5.5rem;
    right: 1.5rem;
}
</style>

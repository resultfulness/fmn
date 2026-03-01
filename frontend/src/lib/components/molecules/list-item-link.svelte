<script lang="ts">
import type { Snippet } from "svelte";

interface ListItemLinkProps {
    children: Snippet;
    href: string;
    ariaLabel?: string;
}

const { children, href, ariaLabel }: ListItemLinkProps = $props();
</script>

<li>
    <a {href} aria-label={ariaLabel} class="text-content">
        {@render children()}
    </a>
</li>

<style>
a {
    background-color: var(--clr-surface);
    border-radius: var(--rounding);
    padding: 0.5rem;
    text-decoration: none;
    display: block;

    &:focus-visible {
        outline: 2px solid var(--clr-primary);
    }

    &:hover {
        cursor: pointer;
    }
}
</style>

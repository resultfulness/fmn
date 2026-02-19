<script lang="ts">
import { page } from "$app/state";
import { ChefHat, Salad, ShoppingBasket } from "@lucide/svelte";
import AppNavigationLink from "./app-navigation-link.svelte";

const linkProps = [
    {
        label: "recipes",
        icon: ChefHat,
        href: "/recipes",
    },
    {
        label: "cart",
        icon: ShoppingBasket,
        href: "/",
    },
    {
        label: "items",
        icon: Salad,
        href: "/items",
    },
];

const path = $derived(page.url.pathname);
</script>

<nav>
    <div class="highlight"></div>
    {#each linkProps as props}
        <AppNavigationLink
            {...props}
            active={props.href === "/"
                ? props.href === path
                : path.startsWith(props.href)}
        />
    {/each}
</nav>

<style>
nav {
    background-color: var(--clr-overlay);
    display: flex;
    padding: 0.5rem;
    gap: 0.5rem;
    position: relative;

    :global(> *) {
        flex: 1;
    }
}

.highlight {
    background-color: oklch(from var(--clr-primary) l c h / 0.2);
    position: absolute;
    position-anchor: --highlight-anchor;
    bottom: anchor(bottom);
    left: anchor(left);
    right: anchor(right);
    top: anchor(top);
    view-transition-name: --highlight;
    border-radius: var(--rounding);
}

::view-transition-group(--highlight) {
    animation-timing-function: cubic-bezier(0.16, 1, 0.3, 1);
}
</style>

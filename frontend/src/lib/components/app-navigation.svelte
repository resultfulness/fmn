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

<nav style:grid-template-columns={`repeat(${linkProps.length}, 1fr)`}>
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
    display: grid;
    padding: 0.5rem;
    gap: 0.5rem;
}
</style>

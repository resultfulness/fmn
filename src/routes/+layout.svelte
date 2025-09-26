<script lang="ts">
import "../app.css";
import { page } from "$app/state";
import Icon from "$lib/components/icon.svelte";

import type { LayoutProps } from "./$types";
let { children }: LayoutProps = $props();

const navLinks = [
    { title: "cart", href: "/cart", icon: "shopping_cart" },
    { title: "items", href: "/items", icon: "nutrition" },
    { title: "recipes", href: "/recipes", icon: "chef_hat" },
];

let path = $derived(page.url.pathname);
</script>

<div class="app">
    <header>
        <Icon name="arrow_back" />
        <h1>fmn</h1>
    </header>
    <main>
        {@render children()}
    </main>
    <footer>
        <nav class="main-nav">
            {#each navLinks as { title, href, icon }}
                <a
                    {href}
                    class="main-nav__link"
                    class:main-nav__link--active={path === href}
                >
                    <Icon name={icon} size={32} filled={path === href} />
                    <span class="main-nav__link__title">{title}</span>
                </a>
            {/each}
        </nav>
    </footer>
</div>

<style>
.app {
    height: 100vh;
    display: flex;
    flex-direction: column;
}

header {
    display: flex;
    gap: 1rem;
    padding: 3.5rem 1rem 1rem;
}

h1 {
    margin: 0;
    font-size: 1rem;
}

main {
    flex: 1;
    padding: 1rem;
    overflow-y: auto;
}

footer {
    background-color: var(--clr-dark);
    padding-bottom: 1rem;
}

.main-nav {
    margin: 0.5rem;
    display: flex;
}

.main-nav__link {
    flex: 1;
    display: grid;
    place-items: center;
    color: var(--clr-text-mute);
    text-decoration: none;
    gap: 0.25rem;
}

.main-nav__link--active {
    color: inherit;
}

.main-nav__link__title {
    font-size: 12px;
}
</style>

<script lang="ts">
import { type Icon as IconType } from "@lucide/svelte";
import { page } from "$app/state";

interface NavbarLinkProps {
    href: string;
    label: string;
    icon: typeof IconType;
}

const { href, label, icon }: NavbarLinkProps = $props();
const path = $derived(page.url.pathname);
</script>

<a {href} class:active={href === "/" ? href === path : path.startsWith(href)}>
    <svelte:boundary>
        {@const Icon = icon}
        <Icon />
    </svelte:boundary>
    {label}
</a>

<style>
a {
    position: relative;
    background-color: var(--clr-s1);
    color: var(--clr-text-mute);
    text-decoration: none;
    display: grid;
    place-items: center;
    gap: 0.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    padding: 0.5rem;
    top: 0px;
    transition:
        top 200ms,
        background-color 200ms,
        color 200ms;
}

.active {
    color: var(--clr-primary);
    top: -4px;
    box-shadow: 0 4px 0 0 var(--clr-primary);
    background-color: var(--clr-s2);
}

a:focus,
a:hover {
    outline: none;
    top: -4px;
}
</style>

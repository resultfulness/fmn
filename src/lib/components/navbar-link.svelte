<script lang="ts">
import { type Icon as IconType } from "@lucide/svelte";
import { page } from "$app/state";

interface NavbarLinkProps {
    href: string;
    label: string;
    icon: typeof IconType;
}

const { href, label, icon }: NavbarLinkProps = $props();
</script>

<a {href} class:active={href === page.url.pathname}>
    <svelte:boundary>
        {@const Icon = icon}
        <Icon />
    </svelte:boundary>
    {label}
</a>

<style>
a {
    color: inherit;
    text-decoration: none;
    display: grid;
    place-items: center;
    font-size: 0.875rem;
    padding: 0.5rem;
}

.active {
    background-color: var(--clr-s2);
}
</style>

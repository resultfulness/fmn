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
    background-color: var(--clr-s1);
    color: var(--clr-text-mute);
    text-decoration: none;
    display: grid;
    place-items: center;
    gap: 0.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    padding: 0.5rem;
}

.active {
    color: var(--clr-primary);
    position: relative;
    top: -4px;
    box-shadow: 0 4px 0 0 var(--clr-primary);
    background-color: var(--clr-s2);
}
</style>

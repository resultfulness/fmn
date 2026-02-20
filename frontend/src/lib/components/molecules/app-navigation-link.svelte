<script lang="ts">
import { type Icon as IconType } from "@lucide/svelte";

interface AppNavigationLinkProps {
    href: string;
    label: string;
    icon: typeof IconType;
    active: boolean;
}

const { href, label, icon, active }: AppNavigationLinkProps = $props();
</script>

<a {href} class:active class="text-subtitle">
    <svelte:boundary>
        {@const Icon = icon}
        <Icon size={32} />
    </svelte:boundary>
    {label}
</a>

<style>
a {
    padding: 0.5rem;
    display: grid;
    justify-items: center;
    text-decoration: none;
    border-radius: var(--rounding);
}

a:focus-visible {
    outline: 2px solid var(--clr-primary);
}

a.active {
    color: var(--clr-primary);
    anchor-name: --highlight-anchor;
}
</style>

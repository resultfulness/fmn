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

<a {href} class:active>
    <svelte:boundary>
        {@const Icon = icon}
        <Icon size={32} />
    </svelte:boundary>
    {label}
</a>

<style>
a {
    padding: 0.5rem;
    color: var(--clr-text-dim);
    font: var(--font-label);
    border-radius: var(--rounding);
    display: grid;
    justify-items: center;
    text-decoration: none;
}

a.active {
    color: var(--clr-primary);
    background-color: oklch(from var(--clr-primary) l c h / 0.2);
}
</style>

<script lang="ts">
import type { Icon as IconType } from "@lucide/svelte";
import Button, { type ButtonProps } from "$lib/components/atoms/button.svelte";

interface IconButtonProps extends Omit<ButtonProps, "children"> {
    icon: typeof IconType;
    size?: number;
}

const { icon, size = 24, ...props }: IconButtonProps = $props();
</script>

<div class="icon-button">
    <Button {...props}>
        <svelte:boundary>
            {@const Icon = icon}
            <Icon {size} />
        </svelte:boundary>
    </Button>
</div>

<style>
.icon-button > :global(.button) {
    padding: 0.5rem;
    border-radius: 100vw;
    aspect-ratio: 1;
}
</style>

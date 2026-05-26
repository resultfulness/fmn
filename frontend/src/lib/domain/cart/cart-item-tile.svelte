<script lang="ts">
import IconTile from "$lib/ui/molecules/icon-tile.svelte";
import type { CartItemDisplay } from "./cart";

interface CartItemTileProps {
    cartItem: CartItemDisplay;
    onclick: (id: number) => void;
}

const { cartItem, onclick }: CartItemTileProps = $props();
</script>

<IconTile
    iconUrl={cartItem.icon}
    label={cartItem.name}
    subtitle={cartItem.quantity
        ? `${cartItem.quantity} ${cartItem.unit}`
        : cartItem.description}
    onclick={() => onclick(cartItem.item_id)}
/>

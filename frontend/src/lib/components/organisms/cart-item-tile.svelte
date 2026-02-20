<script lang="ts">
import type { CartItem } from "$lib/schemas/cart";
import IconTile from "../molecules/icon-tile.svelte";

const { item }: { item: CartItem } = $props();
</script>

<IconTile
    iconUrl={item.icon}
    label={item.name}
    subtitle={item.quantity ? `${item.quantity} ${item.unit}` : undefined}
/>

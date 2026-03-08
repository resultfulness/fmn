<script lang="ts">
import type { CartItem } from "$lib/schemas/cart";
import IconTileGrid from "$lib/components/molecules/icon-tile-grid.svelte";
import type { Item } from "$lib/schemas/items";

interface CartGridProps {
    cart?: CartItem[];
    items?: Item[];
    removeItem: (item_id: number) => void;
}

const { cart, items, removeItem }: CartGridProps = $props();
</script>

{#if cart === undefined}
    loading...
{:else if cart.length <= 0}
    cart is empty
{:else}
    <IconTileGrid
        tiles={cart
            .map(({ item_id, description, quantity }) => {
                const item = items?.find(item => item.item_id === item_id);
                if (!item) return;
                const { icon, name, unit } = item;
                return {
                    iconUrl: icon,
                    label: name,
                    subtitle: quantity ? `${quantity} ${unit}` : description,
                    onclick: () => removeItem(item_id),
                };
            })
            .filter(v => v !== undefined)}
    />
{/if}

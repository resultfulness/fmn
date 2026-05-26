<script lang="ts">
import IconTileGrid from "$lib/ui/molecules/icon-tile-grid.svelte";
import type { Item } from "../items/items";
import type { CartItem } from "./cart";

interface CartGridProps {
    cart: CartItem[];
    items: Item[];
    onCartItemClick: (item_id: number) => void;
}

const { cart, items, onCartItemClick }: CartGridProps = $props();

const tiles = $derived(
    cart
        .map(({ item_id, description, quantity }) => {
            const item = items?.find(item => item.item_id === item_id);
            if (!item) return;
            const { icon, name, unit } = item;
            return {
                iconUrl: icon,
                label: name,
                subtitle: quantity ? `${quantity} ${unit}` : description,
                onclick: () => onCartItemClick(item_id),
            };
        })
        .filter(v => v !== undefined)
);
</script>

<IconTileGrid {tiles} />

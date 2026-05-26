<script lang="ts">
import IconTileGrid from "$lib/ui/molecules/icon-tile-grid.svelte";
import type { Item } from "./items";

interface ItemGridProps {
    items: Item[];
    onItemClick: (item_id: number) => void;
}

const { items, onItemClick }: ItemGridProps = $props();
</script>

<IconTileGrid
    centerLabels
    tiles={items.map(({ item_id, icon, name }) => ({
        iconUrl: icon,
        label: name,
        onclick: () => onItemClick(item_id),
    }))}
/>

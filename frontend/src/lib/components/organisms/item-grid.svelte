<script lang="ts">
import IconTileGrid from "$lib/components/molecules/icon-tile-grid.svelte";
import type { Item } from "$lib/schemas/items";

interface ItemGridProps {
    items?: Item[];
    addItem: (item_id: number) => void;
}

const { items, addItem }: ItemGridProps = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items here
{:else}
    <IconTileGrid
        centerLabels
        tiles={items.map(({ item_id, icon, name }) => ({
            iconUrl: icon,
            label: name,
            onclick: () => addItem(item_id),
        }))}
    />
{/if}

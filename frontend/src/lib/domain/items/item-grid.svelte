<script lang="ts">
import ItemTile from "./item-tile.svelte";
import type { Item } from "./item";

interface ItemGridProps {
    items: Item[];
    onItemClick: (item_id: number) => void;
}

const { items, onItemClick }: ItemGridProps = $props();
</script>

<div class="tile-grid">
    {#each items as item}
        <ItemTile {item} onclick={onItemClick} />
    {/each}
</div>

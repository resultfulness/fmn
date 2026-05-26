<script lang="ts">
import AnchorItem from "$lib/ui/molecules/anchor-item.svelte";
import ItemWide from "./item-wide.svelte";
import type { Item } from "./items";

const { items }: { items: Item[] } = $props();
</script>

<ul>
    {#each items as item}
        <AnchorItem
            href={`/items/${item.item_id}/edit`}
            ariaLabel={`edit item ${item.name}`}
        >
            <ItemWide {item} />
        </AnchorItem>
    {/each}
</ul>

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

<script lang="ts">
import type { Item } from "$lib/schemas/items";
import ListItemLink from "$lib/components/molecules/list-item-link.svelte";
import ItemListItem from "./item-list-item.svelte";

const { items }: { items: Item[] } = $props();
</script>

<ul>
    {#each items as item}
        <ListItemLink
            href={`/items/${item.item_id}/edit`}
            ariaLabel={`edit item ${item.name}`}
        >
            <ItemListItem {item} />
        </ListItemLink>
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

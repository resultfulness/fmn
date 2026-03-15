<script lang="ts">
import type { Item } from "$lib/schemas/items";
import ListItemButton from "$lib/components/molecules/list-item-button.svelte";
import ItemListItem from "./item-list-item.svelte";

interface ItemListProps {
    items: Item[];
    onclick: (id: number) => void;
}

const { items, onclick }: ItemListProps = $props();
</script>

<ul>
    {#each items as item}
        <ListItemButton onclick={() => onclick(item.item_id)}>
            <ItemListItem {item} />
        </ListItemButton>
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

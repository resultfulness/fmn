<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import ListItemButton from "$lib/components/molecules/list-item-button.svelte";
import ItemListItem from "./item-list-item.svelte";

interface ItemListProps {
    items?: ItemShort[];
    onclick: (id: number) => void;
}

const { items, onclick }: ItemListProps = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items here
{:else}
    <ul>
        {#each items as item}
            <ListItemButton onclick={() => onclick(item.item_id)}>
                <ItemListItem {item} />
            </ListItemButton>
        {/each}
    </ul>
{/if}

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

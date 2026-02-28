<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import ListItemLink from "../molecules/list-item-link.svelte";
import ItemListItem from "./item-list-item.svelte";

const { items }: { items?: ItemShort[] } = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items here
{:else}
    <ul>
        {#each items as item}
            <ListItemLink
                href={`/items/edit/${item.item_id}`}
                ariaLabel={`edit item ${item.name}`}
            >
                <ItemListItem {item} />
            </ListItemLink>
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

<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import ItemListEntry from "./item-list-entry.svelte";

const { items }: { items?: ItemShort[] } = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items here
{:else}
    <ul>
        {#each items as item}
            <ItemListEntry {item} />
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

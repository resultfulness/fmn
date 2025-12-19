<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { type ItemShort } from "$lib/schemas/items";
import Button from "$lib/components/button.svelte";
import ItemListRow from "$lib/components/item-list-row.svelte";
import { Plus, Search } from "@lucide/svelte";
import Input from "$lib/components/input.svelte";
import { HeaderState } from "$lib/components/header.svelte";

let items = $state<ItemShort[]>();
let searchterm = $state("");
const itemsFiltered = $derived(
    items?.filter(({ name }) => name.includes(searchterm))
);

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => alert(e));
});

function handleDeleteItem(item_id: number) {
    api.items
        .delete(item_id)
        .then(() => (items = items?.filter(item => item.item_id !== item_id)))
        .catch(e => alert(e));
}

HeaderState.title = "items";
</script>

<button
    style="position: fixed; top: 0; right: 0;"
    onclick={() => {
        localStorage.removeItem("items");
        window.location.reload();
    }}
>
    clear ls
</button>
<div class="search">
    <Input
        icon={Search}
        clearable
        placeholder="search for items..."
        bind:value={searchterm}
    />
</div>
<div class="items">
    {#if itemsFiltered === undefined}
        loading...
    {:else if itemsFiltered.length <= 0}
        no items{#if searchterm}&nbsp;containing '{searchterm}'{/if}... :&lt;
    {:else}
        <ul>
            {#each itemsFiltered as item}
                <ItemListRow {item} handleDelete={handleDeleteItem} />
            {/each}
        </ul>
    {/if}
</div>
<div class="actions">
    <Button square href="/items/new">
        <Plus size={40} />
    </Button>
</div>

<style>
.search {
    padding-inline: 1rem;
}

.items {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
}

ul {
    list-style-type: none;
    background-color: var(--clr-outline);
    display: grid;
    gap: 1px;
    padding: 1px;
    margin: 0;
}

.actions {
    background-color: var(--clr-base);
    display: grid;
    padding: 0.5rem;
}
</style>

<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { ItemUpdate, type ItemShort } from "$lib/schemas/items";
import Button from "$lib/components/button.svelte";
import ItemListRow from "$lib/components/item-list-row.svelte";
import { Plus } from "@lucide/svelte";

let items = $state<ItemShort[]>();

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

function handleUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    const form = new FormData(e.target as HTMLFormElement);

    const item = ItemUpdate.safeParse({
        name: form.get("name"),
        icon: form.get("icon"),
    });

    if (!item.success) {
        alert(item.error);
        return;
    }

    api.items
        .update(+form.get("id")!, item.data)
        .then(
            item =>
                (items![
                    items?.findIndex(_item => _item.item_id === item.item_id)!
                ] = item)
        )
        .catch(e => alert(e));
}
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
{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items... :&lt;
{:else}
    <ul>
        {#each items as item}
            <ItemListRow {item} handleDelete={handleDeleteItem} />
        {/each}
    </ul>
{/if}
<Button fab href="/items/new">
    <Plus size={40} />
</Button>

<style>
ul {
    list-style-type: none;
    background-color: var(--clr-outline);
    display: grid;
    gap: 1px;
    padding: 1px;
    margin: 0;
    margin-bottom: 6rem;
}
</style>

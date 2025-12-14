<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { ItemCreate, ItemUpdate, type ItemShort } from "$lib/schemas/items";

let items = $state<ItemShort[]>();

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => alert(e));
});

function handleCreateItem(e: SubmitEvent) {
    e.preventDefault();
    const form = new FormData(e.target as HTMLFormElement);

    const item = ItemCreate.safeParse({
        name: form.get("name"),
        icon: form.get("icon"),
    });

    if (!item.success) {
        alert(item.error);
        return;
    }

    api.items
        .create(item.data)
        .then(item => items?.push(item))
        .catch(e => alert(e));
}

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

<form onsubmit={handleCreateItem}>
    <input type="text" name="name" />
    <input type="text" name="icon" />
    <button>create</button>
</form>

{#if items === undefined}
    loading...
{:else}
    <form onsubmit={handleUpdateItem}>
        editing
        <select name="id">
            {#each items as item}
                <option value={item.item_id}>
                    {item.name} (id: {item.item_id})
                </option>
            {/each}
        </select>
        <input type="text" name="name" />
        <input type="text" name="icon" />
        <button>update</button>
    </form>

    <ul>
        {#each items as item}
            <li>
                {item.name}
                <button onclick={() => handleDeleteItem(item.item_id)}>
                    delete
                </button>
            </li>
        {/each}
    </ul>
{/if}

<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/button.svelte";
import { HeaderState } from "$lib/components/header.svelte";
import Input from "$lib/components/input.svelte";
import { ItemUpdate } from "$lib/schemas/items";

let { data } = $props();
let { item } = $derived(data);
let name = $derived(item?.name ?? "");
let icon = $derived(item?.icon ?? "");

function handleUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    const itemUpdate = ItemUpdate.safeParse({ name, icon });

    if (!itemUpdate.success) {
        alert(itemUpdate.error);
        return;
    }

    api.items
        .update(item.item_id, itemUpdate.data)
        .then(() => goto("/items"))
        .catch(e => alert(e));
}

HeaderState.title = "editing item ";
HeaderState.backUrl = "/items";
</script>

<div class="page">
    <form onsubmit={handleUpdateItem}>
        <Input type="text" bind:value={name} placeholder="item name" />
        <Input type="text" bind:value={icon} placeholder="item icon" />
        <Button variant="primary">update</Button>
    </form>

    <h2>icon preview</h2>
    <img src={icon} alt="couldn't load preview" />
</div>

<style>
.page {
    padding: 1rem;
}

form {
    display: grid;
    gap: 0.25rem;
}

img {
    border: 1px solid var(--clr-primary);
    width: calc(100% - 8rem);
    margin-inline: auto;
    display: block;
    text-align: center;
}
</style>

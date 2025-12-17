<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/button.svelte";
import { HeaderState } from "$lib/components/header.svelte";
import Input from "$lib/components/input.svelte";
import { ItemCreate } from "$lib/schemas/items";

let name = $state("");
let icon = $state("");

function createItem() {
    const item = ItemCreate.safeParse({ name, icon });

    if (!item.success) {
        alert(item.error);
        return;
    }

    api.items
        .create(item.data)
        .then(() => {
            name = "";
            icon = "";
        })
        .catch(e => alert(e));
}

function handleCreateItem(e: SubmitEvent) {
    e.preventDefault();
    createItem();
}

function handleCreateItemAndGoBack() {
    createItem();
    goto("/items");
}

HeaderState.title = "new item";
HeaderState.backUrl = "/items";
</script>

<form onsubmit={handleCreateItem}>
    <Input type="text" name="name" bind:value={name} placeholder="item name" />
    <Input type="text" name="icon" bind:value={icon} placeholder="item icon" />
    <div>
        <Button variant="secondary">create & stay</Button>
        <Button
            variant="primary"
            onclick={handleCreateItemAndGoBack}
            type="button"
        >
            create
        </Button>
    </div>
</form>

<h2>icon preview</h2>
<img src={icon} alt="couldn't load preview" />

<style>
form {
    display: grid;
    gap: 0.25rem;
}

form div {
    display: grid;
    gap: 0.25rem;
    grid-template-columns: repeat(2, 1fr);
}

img {
    border: 1px solid var(--clr-primary);
    width: calc(100% - 8rem);
    margin-inline: auto;
    display: block;
    text-align: center;
}
</style>

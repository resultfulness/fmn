<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/header.svelte";
import Input from "$lib/components/atoms/input.svelte";
import { ItemCreate } from "$lib/schemas/items";
import InputField from "$lib/components/input-field.svelte";

let name = $state("");
let icon = $state("");
let unit = $state("");

function createItem() {
    const item = ItemCreate.safeParse({ name, icon, unit });

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

HeaderState.title = "";
HeaderState.backUrl = "/items";
</script>

<div class="page">
    <img src={icon} alt="" />
    <h1>{name.length > 0 ? name : "new item"}</h1>
    <form onsubmit={handleCreateItem}>
        <InputField
            type="text"
            name="name"
            bind:value={name}
            label="Name"
        />
        <InputField
            type="text"
            name="icon"
            bind:value={icon}
            label="Icon"
        />
        <InputField
            type="text"
            name="unit"
            bind:value={unit}
            label="Unit"
        />
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
</div>

<style>
.page {
    padding: 1rem;
    padding-top: 2rem;
    display: grid;
    gap: 1rem;
}

h1 {
    font: var(--font-title);
    margin: 0;
    text-align: center;
}

form {
    display: grid;
    gap: 1rem;
}

form div {
    display: grid;
    gap: 1rem;
    padding-top: 1rem;
    grid-template-columns: repeat(2, 1fr);
}

img {
    position: relative;
    width: 128px;
    aspect-ratio: 1;
    object-fit: cover;
    margin-inline: auto;
}
</style>

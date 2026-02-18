<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/header.svelte";
import { ItemCreate } from "$lib/schemas/items";
import InputField from "$lib/components/input-field.svelte";
import DropdownField from "$lib/components/dropdown-field.svelte";
import Title from "$lib/components/atoms/title.svelte";

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
    <Title>{name.length > 0 ? name : "new item"}</Title>
    <form onsubmit={handleCreateItem}>
        <InputField
            type="text"
            name="name"
            bind:value={name}
            label="Name"
            placeholder="enter name..."
        />
        <InputField
            type="text"
            name="icon"
            bind:value={icon}
            label="Icon URL"
            placeholder="enter url..."
        />
        <DropdownField
            options={["tbsp", "pcs", "g"]}
            bind:value={unit}
            label="Unit"
            placeholder="pick a unit..."
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

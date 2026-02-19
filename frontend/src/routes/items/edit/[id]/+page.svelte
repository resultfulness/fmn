<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/header.svelte";
import { ItemUpdate } from "$lib/schemas/items";
import DropdownField from "$lib/components/dropdown-field.svelte";
import InputField from "$lib/components/input-field.svelte";
import Title from "$lib/components/atoms/title.svelte";

let { data } = $props();
let { item } = $derived(data);
let { name, unit, icon } = $derived(item);

function handleUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    const itemUpdate = ItemUpdate.safeParse({ name, icon, unit });

    if (!itemUpdate.success) {
        alert(itemUpdate.error);
        return;
    }

    api.items
        .update(item.item_id, itemUpdate.data)
        .then(() => goto("/items"))
        .catch(e => alert(e));
}

async function handleDeleteItem() {
    askForConfirmation(
        `deleting ${item.name}`,
        "are you sure you want to remove this item?"
    ).then(yes => {
        if (yes) {
            api.items
                .delete(item.item_id)
                .then(() => goto("/items"))
                .catch(e => alert(e));
        }
    });
}

HeaderState.title = "";
HeaderState.backUrl = "/items";
</script>

<div class="page">
    <img src={icon} alt="" />
    <Title>
        {#if name.length > 0}
            {name}
        {:else}
            &nbsp;
        {/if}
    </Title>
    <form onsubmit={handleUpdateItem}>
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
            options={["tbsp", "pcs", "pkgs"]}
            bind:value={unit}
            label="Unit"
            placeholder="pick a unit..."
        />
        <div>
            <Button variant="danger" type="button" onclick={handleDeleteItem}>
                delete
            </Button>
            <Button variant="primary">save</Button>
        </div>
    </form>
</div>

<style>
.page {
    padding: 1rem;
    padding-top: 2rem;
    display: grid;
    gap: 1rem;
    overflow-y: auto;
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

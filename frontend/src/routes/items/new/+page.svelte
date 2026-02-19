<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import { HeaderState } from "$lib/components/header.svelte";
import { ItemCreate } from "$lib/schemas/items";
import Title from "$lib/components/atoms/title.svelte";
import ItemForm from "$lib/components/item-form.svelte";
import Button from "$lib/components/atoms/button.svelte";

let item: ItemCreate = $state({ name: "", icon: "", unit: "" });

async function createItem() {
    const itemCreate = ItemCreate.safeParse(item);

    if (!itemCreate.success) {
        throw itemCreate.error;
    }

    api.items
        .create(itemCreate.data)
        .then(() => (item = { name: "", icon: "", unit: "" }))
        .catch(e => {
            throw e;
        });
}

function handleCreateItem(e: SubmitEvent) {
    e.preventDefault();
    createItem()
        .then(() => goto("/items"))
        .catch(e => alert(e));
}

function handleCreateItemAndStay() {
    createItem().catch(e => alert(e));
}

HeaderState.title = "";
HeaderState.backUrl = "/items";
</script>

<div class="page">
    <img src={item.icon} alt="" />
    <Title>{item.name.length > 0 ? item.name : "new item"}</Title>
    <ItemForm onsubmit={handleCreateItem} bind:item>
        {#snippet actions()}
            <Button
                variant="secondary"
                type="button"
                onclick={handleCreateItemAndStay}
            >
                create & stay
            </Button>
            <Button>create</Button>
        {/snippet}
    </ItemForm>
</div>

<style>
.page {
    padding: 1rem;
    padding-top: 2rem;
    display: grid;
    gap: 1rem;
    overflow-y: auto;
}

img {
    position: relative;
    width: 128px;
    aspect-ratio: 1;
    object-fit: cover;
    margin-inline: auto;
}
</style>

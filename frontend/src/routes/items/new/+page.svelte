<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import { ItemCreate } from "$lib/schemas/items";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemForm from "$lib/components/organisms/item-form.svelte";

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
    <h2 class="text-title line-clamp-2">
        {item.name.length > 0 ? item.name : "new item"}
    </h2>
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
    overflow-y: auto;

    > :global(* + *) {
        margin-top: 1rem;
    }
}

h2 {
    text-align: center;
    margin: 0;
    flex: 1;
}

img {
    position: relative;
    width: 128px;
    aspect-ratio: 1;
    object-fit: cover;
    margin-inline: auto;
    display: block;
}
</style>

<script lang="ts">
import { goto } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { ItemUpdate } from "$lib/schemas/items";
import { askForConfirmation } from "$lib/components/confirm.svelte";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemForm from "$lib/components/organisms/item-form.svelte";

let { data } = $props();
let item = $derived(proxify(data.item));

function handleUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    const itemUpdate = ItemUpdate.safeParse(item);

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
    <img src={item.icon} alt="" />
    <h2 class="text-title line-clamp-2">
        {#if item.name.length > 0}
            {item.name}
        {:else}
            &nbsp;
        {/if}
    </h2>
    <ItemForm onsubmit={handleUpdateItem} bind:item>
        {#snippet actions()}
            <Button variant="danger" type="button" onclick={handleDeleteItem}>
                delete
            </Button>
            <Button>save</Button>
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
}

img {
    position: relative;
    width: 128px;
    aspect-ratio: 1;
    object-fit: cover;
    margin-inline: auto;
}
</style>

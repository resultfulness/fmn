<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import ItemForm from "$lib/domain/items/item-form.svelte";
import { ItemCreate } from "$lib/domain/items/item";
import { toastIssues } from "$lib/error";
import Button from "$lib/ui/elements/button.svelte";
import { HeaderState } from "$lib/ui/header.svelte";
import FormPage from "$lib/ui/templates/form-page.svelte";
import { pushToast } from "$lib/ui/toast.svelte";
import { onMount } from "svelte";

let maybeItem: Partial<ItemCreate> = $state({});

async function createItem() {
    const itemCreate = ItemCreate.safeParse(maybeItem);

    if (!itemCreate.success) {
        throw itemCreate.error.issues;
    }

    api.items
        .create(itemCreate.data)
        .catch(e => pushToast(e, "error"));
}

function handleCreateItem(e: SubmitEvent) {
    e.preventDefault();
    createItem()
        .then(() => goto("/items"))
        .then(() => pushToast("item added", "success"))
        .catch(toastIssues);
}

function handleCreateItemAndStay() {
    createItem().catch(toastIssues);
}

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = "/items";
});
</script>

<FormPage
    icon={maybeItem.icon}
    title={maybeItem.name && maybeItem.name.length > 0
        ? maybeItem.name
        : "new item"}
>
    <ItemForm onsubmit={handleCreateItem} bind:item={maybeItem}>
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
</FormPage>

<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import { ItemCreate } from "$lib/schemas/items";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemForm from "$lib/components/organisms/item-form.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";
import { pushToast } from "$lib/components/toast.svelte";

let maybeItem: Partial<ItemCreate> = $state({});

async function createItem() {
    const itemCreate = ItemCreate.safeParse(maybeItem);

    if (!itemCreate.success) {
        throw itemCreate.error;
    }

    api.items
        .create(itemCreate.data)
        .then(() => (maybeItem = {}))
        .catch(e => {
            throw e;
        });
}

function handleCreateItem(e: SubmitEvent) {
    e.preventDefault();
    createItem()
        .then(() => goto("/items"))
        .catch(e => pushToast(e, "error"));
}

function handleCreateItemAndStay() {
    createItem().catch(e => pushToast(e, "error"));
}

HeaderState.title = "";
HeaderState.backUrl = "/items";
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

<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import { ItemCreate, ItemUpdate } from "$lib/schemas/items";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemForm from "$lib/components/organisms/item-form.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";

let item = $state<ItemUpdate>({});

async function createItem() {
    const itemCreate = ItemCreate.safeParse(item);

    if (!itemCreate.success) {
        throw itemCreate.error;
    }

    api.items
        .create(itemCreate.data)
        .then(() => (item = {}))
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

<FormPage
    icon={item.icon}
    title={item.name && item.name.length > 0 ? item.name : "new item"}
>
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
</FormPage>

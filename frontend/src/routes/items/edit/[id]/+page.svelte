<script lang="ts">
import { goto, invalidateAll } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { ItemUpdate } from "$lib/schemas/items";
import { askForConfirmation } from "$lib/components/confirm.svelte";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemForm from "$lib/components/organisms/item-form.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";

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
        .then(invalidateAll)
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

<FormPage icon={data.item.icon} title={data.item.name}>
    <ItemForm onsubmit={handleUpdateItem} bind:item>
        {#snippet actions()}
            <Button variant="danger" type="button" onclick={handleDeleteItem}>
                delete
            </Button>
            <Button>save</Button>
        {/snippet}
    </ItemForm>
</FormPage>

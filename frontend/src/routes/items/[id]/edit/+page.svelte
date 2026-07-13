<script lang="ts">
import { goto, invalidateAll } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { askForConfirmation } from "$lib/ui/confirm.svelte";
import { pushToast } from "$lib/ui/toast";
import { toastIssues } from "$lib/error.js";
import { onMount } from "svelte";
import { HeaderState } from "$lib/ui/header.svelte";
import { ItemUpdate } from "$lib/domain/items/item";
import FormPage from "$lib/ui/templates/form-page.svelte";
import ItemForm from "$lib/domain/items/item-form.svelte";
import Button from "$lib/ui/elements/button.svelte";

let { data } = $props();

let item = $derived(proxify(data.item));

async function handleUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    const itemUpdate = ItemUpdate.safeParse(item);

    if (!itemUpdate.success) {
        toastIssues(itemUpdate.error.issues);
        return;
    }

    await api.items.update(item.item_id, itemUpdate.data);
    pushToast("item updated", "success");
    await invalidateAll();
}

async function handleDeleteItem() {
    askForConfirmation(
        `deleting ${item.name}`,
        "are you sure you want to remove this item?"
    )
        .then(async () => {
            await api.items.delete(item.item_id);
            pushToast("item deleted", "success");
            await goto("/items");
        })
        .catch(() => {});
}

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = "/items";
});
</script>

<FormPage icon={item.icon} title={item.name}>
    <ItemForm onsubmit={handleUpdateItem} bind:item>
        {#snippet actions()}
            <Button variant="danger" type="button" onclick={handleDeleteItem}>
                delete
            </Button>
            <Button>save</Button>
        {/snippet}
    </ItemForm>
</FormPage>

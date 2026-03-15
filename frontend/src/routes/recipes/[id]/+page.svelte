<script lang="ts">
import api from "$lib/api";
import { onMount } from "svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";
import type { Item } from "$lib/schemas/items";
import { pushToast } from "$lib/components/toast.svelte";
import Textarea from "$lib/components/atoms/textarea.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import { Pencil } from "@lucide/svelte";
import RecipeIngredientsList from "$lib/components/organisms/recipe-ingredients-list.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import Button from "$lib/components/atoms/button.svelte";

let { data } = $props();
let items: Item[] | undefined = $state();

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => pushToast(e, "error"));
});

HeaderState.title = "";
HeaderState.backUrl = "/recipes";
</script>

<FormPage icon={data.recipe.icon} title={data.recipe.name}>
    <div class="text-content text-center">
        {data.recipe.servings} servings
    </div>
    <h3 class="text-heading">Ingredients</h3>
    {#if items && data.recipe.items.length > 0}
        <RecipeIngredientsList
            ingredients={data.recipe.items.map(({ item_id, quantity }) => ({
                item: items!.find(item => item.item_id === item_id)!,
                quantity,
            }))}
        />
    {/if}
    <h3 class="text-heading">Description</h3>
    <Textarea readonly value={data.recipe.description} />
    <div class="edit">
        <Button href={`/recipes/${data.recipe.recipe_id}/edit`}>
            <Pencil /> edit recipe
        </Button>
    </div>
</FormPage>

<style>
h3 {
    margin: 0;
}

.edit {
    display: grid;
    margin-bottom: 1rem;
}
</style>

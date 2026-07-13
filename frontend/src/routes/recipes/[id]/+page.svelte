<script lang="ts">
import { onMount } from "svelte";
import { HeaderState } from "$lib/ui/header.svelte";
import FormPage from "$lib/ui/templates/form-page.svelte";
import Textarea from "$lib/ui/elements/textarea.svelte";
import Button from "$lib/ui/elements/button.svelte";
import { Pencil } from "@lucide/svelte";
import RecipeIngredientsList from "$lib/domain/recipes/recipe-ingredients-list.svelte";
import itemStore from "$lib/domain/items/store.svelte.js";

let { data } = $props();

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = "/recipes";

    itemStore.load();
});
</script>

<FormPage icon={data.recipe.icon} title={data.recipe.name}>
    <div class="text-content text-center">
        {data.recipe.servings} servings
    </div>
    <h3 class="text-heading">Ingredients</h3>
    {#if data.recipe.items.length > 0}
        <RecipeIngredientsList
            ingredients={data.recipe.items.map(({ item_id, quantity }) => ({
                item: itemStore.items.find(item => item.item_id === item_id)!,
                quantity,
            }))}
        />
    {:else}
        <div class="text-subtitle text-center">no ingredients yet!</div>
    {/if}
    <h3 class="text-heading">Description</h3>
    {#if data.recipe.description.length > 0}
        <Textarea readonly growing value={data.recipe.description} />
    {:else}
        <div class="text-subtitle text-center">no description yet!</div>
    {/if}
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
    margin-top: 2rem;
    margin-bottom: 1rem;
}
</style>

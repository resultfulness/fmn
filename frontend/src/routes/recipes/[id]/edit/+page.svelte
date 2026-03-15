<script lang="ts">
import { goto, invalidateAll } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { askForConfirmation } from "$lib/components/confirm.svelte";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";
import { pushToast } from "$lib/components/toast.svelte";
import { RecipeUpdate } from "$lib/schemas/recipes.js";
import RecipeUpdateForm from "$lib/components/organisms/recipe-update-form.svelte";
import { onMount } from "svelte";
import type { Item } from "$lib/schemas/items.js";
import { printIssues } from "$lib/error.js";

let { data } = $props();
let recipe = $derived(proxify(data.recipe));
let items: Item[] | undefined = $state();

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => pushToast(e, "error"));
});

function handleUpdateRecipe(e: SubmitEvent) {
    e.preventDefault();
    const recipeUpdate = RecipeUpdate.safeParse(recipe);

    if (!recipeUpdate.success) {
        printIssues(recipeUpdate.error.issues);
        return;
    }

    api.recipes
        .update(recipe.recipe_id, recipeUpdate.data)
        .then(invalidateAll)
        .catch(e => pushToast(e, "error"));
}

async function handleDeleteRecipe() {
    askForConfirmation(
        `deleting ${recipe.name}`,
        "are you sure you want to remove this recipe?"
    ).then(yes => {
        if (yes) {
            api.recipes
                .delete(recipe.recipe_id)
                .then(() => goto("/recipes"))
                .catch(e => pushToast(e, "error"));
        }
    });
}

HeaderState.title = "";
HeaderState.backUrl = `/recipes/${data.recipe.recipe_id}`;
</script>

<FormPage icon={data.recipe.icon} title={data.recipe.name}>
    {#if items}
        <RecipeUpdateForm onsubmit={handleUpdateRecipe} bind:recipe {items}>
            {#snippet actions()}
                <Button
                    variant="danger"
                    type="button"
                    onclick={handleDeleteRecipe}
                >
                    delete
                </Button>
                <Button>save</Button>
            {/snippet}
        </RecipeUpdateForm>
    {/if}
</FormPage>

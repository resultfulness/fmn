<script lang="ts">
import { goto, invalidateAll } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { askForConfirmation } from "$lib/ui/confirm.svelte";
import { pushToast } from "$lib/ui/toast.svelte";
import { HeaderState } from "$lib/ui/header.svelte";
import { onMount } from "svelte";
import { toastIssues } from "$lib/error.js";
import type { Item } from "$lib/domain/items/item";
import { RecipeUpdate } from "$lib/domain/recipes/recipe";
import FormPage from "$lib/ui/templates/form-page.svelte";
import Button from "$lib/ui/elements/button.svelte";
import RecipeUpdateForm from "$lib/domain/recipes/recipe-update-form.svelte";

let { data } = $props();

let items: Item[] | undefined = $state();

let recipe = $derived(proxify(data.recipe));

function handleUpdateRecipe(e: SubmitEvent) {
    e.preventDefault();
    const recipeUpdate = RecipeUpdate.safeParse(recipe);

    if (!recipeUpdate.success) {
        toastIssues(recipeUpdate.error.issues);
        return;
    }

    api.recipes
        .update(recipe.recipe_id, recipeUpdate.data)
        .then(invalidateAll)
        .then(() => pushToast("recipe updated", "success"))
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
                .then(() => pushToast("recipe deleted", "success"))
                .catch(e => pushToast(e, "error"));
        }
    });
}

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = `/recipes/${data.recipe.recipe_id}`;

    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => pushToast(e, "error"));
});
</script>

<FormPage icon={recipe.icon} title={recipe.name}>
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

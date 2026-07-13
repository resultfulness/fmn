<script lang="ts">
import { goto, invalidateAll } from "$app/navigation";
import { proxify } from "$lib/reactivity.svelte";
import api from "$lib/api";
import { askForConfirmation } from "$lib/ui/confirm.svelte";
import { pushToast } from "$lib/ui/toast";
import { HeaderState } from "$lib/ui/header.svelte";
import { onMount } from "svelte";
import { toastIssues } from "$lib/error.js";
import { RecipeUpdate } from "$lib/domain/recipes/recipe";
import FormPage from "$lib/ui/templates/form-page.svelte";
import Button from "$lib/ui/elements/button.svelte";
import RecipeUpdateForm from "$lib/domain/recipes/recipe-update-form.svelte";
import itemStore from "$lib/domain/items/store.svelte.js";

let { data } = $props();

let recipe = $derived(proxify(data.recipe));

async function handleUpdateRecipe(e: SubmitEvent) {
    e.preventDefault();
    const recipeUpdate = RecipeUpdate.safeParse(recipe);

    if (!recipeUpdate.success) {
        toastIssues(recipeUpdate.error.issues);
        return;
    }

    await api.recipes.update(recipe.recipe_id, recipeUpdate.data);
    pushToast("recipe updated", "success");
    await invalidateAll();
}

async function handleDeleteRecipe() {
    askForConfirmation(
        `deleting ${recipe.name}`,
        "are you sure you want to remove this recipe?"
    )
        .then(async () => {
            await api.recipes.delete(recipe.recipe_id);
            pushToast("recipe deleted", "success");
            await goto("/recipes");
        })
        .catch(() => {});
}

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = `/recipes/${data.recipe.recipe_id}`;

    itemStore.load();
});
</script>

<FormPage icon={recipe.icon} title={recipe.name}>
    <RecipeUpdateForm
        onsubmit={handleUpdateRecipe}
        bind:recipe
        items={itemStore.items}
    >
        {#snippet actions()}
            <Button variant="danger" type="button" onclick={handleDeleteRecipe}>
                delete
            </Button>
            <Button>save</Button>
        {/snippet}
    </RecipeUpdateForm>
</FormPage>

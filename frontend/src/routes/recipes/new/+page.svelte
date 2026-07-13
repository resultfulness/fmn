<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import { pushToast } from "$lib/ui/toast";
import { toastIssues } from "$lib/error";
import { onMount } from "svelte";
import { RecipeCreate } from "$lib/domain/recipes/recipe";
import { HeaderState } from "$lib/ui/header.svelte";
import FormPage from "$lib/ui/templates/form-page.svelte";
import RecipeCreateForm from "$lib/domain/recipes/recipe-create-form.svelte";
import Button from "$lib/ui/elements/button.svelte";

let maybeRecipe: Partial<RecipeCreate> = $state({});

onMount(() => {
    HeaderState.title = "";
    HeaderState.backUrl = "/recipes";
});

async function createRecipe() {
    const recipeCreate = RecipeCreate.safeParse(maybeRecipe);

    if (!recipeCreate.success) {
        toastIssues(recipeCreate.error.issues);
        return;
    }

    const recipe = await api.recipes.create(recipeCreate.data)
    return recipe.recipe_id;
}

async function handleCreateRecipe(e: SubmitEvent) {
    e.preventDefault();
    const id = await createRecipe()
    pushToast("recipe added", "success");
    await goto(`/recipes/${id}/edit`);
}
</script>

<FormPage
    icon={maybeRecipe.icon}
    title={maybeRecipe.name && maybeRecipe.name.length > 0
        ? maybeRecipe.name
        : "new recipe"}
>
    <RecipeCreateForm onsubmit={handleCreateRecipe} bind:recipe={maybeRecipe}>
        {#snippet actions()}
            <Button>create</Button>
        {/snippet}
    </RecipeCreateForm>
</FormPage>

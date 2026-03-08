<script lang="ts">
import { goto } from "$app/navigation";
import api from "$lib/api";
import Button from "$lib/components/atoms/button.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import RecipeCreateForm from "$lib/components/organisms/recipe-create-form.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";
import { pushToast } from "$lib/components/toast.svelte";
import { RecipeCreate } from "$lib/schemas/recipes";

let maybeRecipe: Partial<RecipeCreate> = $state({});

async function createRecipe() {
    const recipeCreate = RecipeCreate.safeParse(maybeRecipe);

    if (!recipeCreate.success) {
        throw recipeCreate.error;
    }

    return api.recipes
        .create(recipeCreate.data)
        .then(recipe => {
            maybeRecipe = {};
            return recipe.recipe_id;
        })
        .catch(e => {
            throw e;
        });
}

function handleCreateRecipe(e: SubmitEvent) {
    e.preventDefault();
    createRecipe()
        .then(id => goto(`/recipes/${id}/edit`))
        .catch(e => pushToast(e, "error"));
}

HeaderState.title = "";
HeaderState.backUrl = "/recipes";
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

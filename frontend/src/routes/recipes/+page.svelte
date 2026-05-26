<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { pushToast } from "$lib/ui/toast.svelte";
import type { Recipe } from "$lib/domain/recipes/recipes";
import { HeaderState } from "$lib/ui/header.svelte";
import ListPage from "$lib/ui/templates/list-page.svelte";
import FooterExtension from "$lib/ui/molecules/footer-extension.svelte";
import IconButton from "$lib/ui/molecules/icon-button.svelte";
import Search from "$lib/ui/molecules/search.svelte";
import RecipeAnchorList from "$lib/domain/recipes/recipe-anchor-list.svelte";
import { Plus } from "@lucide/svelte";

let recipes: Recipe[] | undefined = $state();
let searchterm = $state("");

const recipeFound = (recipe: Recipe) =>
    recipe.name.toLowerCase().includes(searchterm.toLowerCase());
const recipesFiltered = $derived(recipes?.filter(recipeFound));

onMount(() => {
    HeaderState.title = "recipes";
    delete HeaderState.backUrl;

    api.recipes
        .readAll()
        .then(_recipes => (recipes = _recipes))
        .catch(e => pushToast(e, "error"));
});
</script>

<ListPage>
    {#if recipesFiltered && recipesFiltered.length > 0}
        <RecipeAnchorList recipes={recipesFiltered} />
    {:else}
        <div class="text-subtitle text-center">
            {#if searchterm}
                no recipes matching {searchterm}
            {:else}
                no recipes!
            {/if}
        </div>
    {/if}
</ListPage>
<FooterExtension>
    <Search bind:searchterm placeholder="search for recipes..." />
    <IconButton href="/recipes/new" icon={Plus} size={32} />
</FooterExtension>

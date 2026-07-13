<script lang="ts">
import { onMount } from "svelte";
import type { Recipe } from "$lib/domain/recipes/recipe";
import { HeaderState } from "$lib/ui/header.svelte";
import ListPage from "$lib/ui/templates/list-page.svelte";
import FooterExtension from "$lib/ui/molecules/footer-extension.svelte";
import IconButton from "$lib/ui/molecules/icon-button.svelte";
import Search from "$lib/ui/molecules/search.svelte";
import RecipeAnchorList from "$lib/domain/recipes/recipe-anchor-list.svelte";
import { Plus } from "@lucide/svelte";
import recipeStore from "$lib/domain/recipes/store.svelte";

let searchterm = $state("");

const recipeFound = (recipe: Recipe) =>
    recipe.name.toLowerCase().includes(searchterm.toLowerCase());
const recipesFiltered = $derived(recipeStore.recipes.filter(recipeFound));

onMount(() => {
    HeaderState.title = "recipes";
    delete HeaderState.backUrl;

    recipeStore.load();
});
</script>

<ListPage>
    {#if recipesFiltered.length > 0}
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

<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import type { Recipe } from "$lib/schemas/recipes";
import { Plus } from "@lucide/svelte";
import Search from "$lib/components/molecules/search.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import RecipeList from "$lib/components/organisms/recipe-list.svelte";
import { pushToast } from "$lib/components/toast.svelte";
import ListPage from "$lib/components/templates/list-page.svelte";

let recipes = $state<Recipe[]>();
let searchterm = $state("");

const recipesSearched = $derived(
    recipes?.filter(({ name }) => name.includes(searchterm))
);

onMount(() => {
    api.recipes
        .readAll()
        .then(_recipes => (recipes = _recipes))
        .catch(e => pushToast(e, "error"));
});

HeaderState.title = "recipes";
delete HeaderState.backUrl;
</script>

<ListPage>
    {#if recipesSearched && recipesSearched.length > 0}
        <RecipeList recipes={recipesSearched} />
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

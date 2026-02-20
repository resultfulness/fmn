<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { type RecipeShort } from "$lib/schemas/recipes";
import { Plus } from "@lucide/svelte";
import Search from "$lib/components/molecules/search.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import RecipeList from "$lib/components/organisms/recipe-list.svelte";

let recipes = $state<RecipeShort[]>();
let searchterm = $state("");

const recipesSearched = $derived(
    recipes?.filter(({ name }) => name.includes(searchterm))
);

onMount(() => {
    api.recipes
        .readAll()
        .then(_recipes => (recipes = _recipes))
        .catch(e => alert(e));
});

HeaderState.title = "recipes";
delete HeaderState.backUrl;
</script>

<div class="recipes">
    <RecipeList recipes={recipesSearched} />
</div>
<FooterExtension>
    <Search bind:searchterm />
    <IconButton href="/recipes/new" icon={Plus} size={32} />
</FooterExtension>

<style>
.recipes {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
}
</style>

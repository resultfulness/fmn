<script lang="ts">
import AnchorItem from "$lib/ui/molecules/anchor-item.svelte";
import RecipeWide from "./recipe-wide.svelte";
import type { Recipe } from "./recipes";

const { recipes }: { recipes: Recipe[] } = $props();
</script>

<ul>
    {#each recipes as recipe}
        <AnchorItem
            href={`/recipes/${recipe.recipe_id}`}
            ariaLabel={`edit recipe ${recipe.name}`}
        >
            <RecipeWide {recipe} />
        </AnchorItem>
    {/each}
</ul>

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

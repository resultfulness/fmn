<script lang="ts">
import type { Recipe } from "./recipe";
import RecipeTile from "./recipe-tile.svelte";

interface RecipeGridProps {
    recipes: Recipe[];
    onRecipeClick: (recipe_id: number) => void;
}

const { recipes, onRecipeClick }: RecipeGridProps = $props();
</script>

<div class="tile-grid">
    {#each recipes as recipe}
        <RecipeTile {recipe} onclick={onRecipeClick} />
    {/each}
</div>

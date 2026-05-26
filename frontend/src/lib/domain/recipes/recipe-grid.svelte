<script lang="ts">
import IconTileGrid from "$lib/ui/molecules/icon-tile-grid.svelte";
import type { Recipe } from "./recipes";

interface RecipeGridProps {
    recipes: Recipe[];
    onRecipeClick: (recipe_id: number) => void;
}

const { recipes, onRecipeClick }: RecipeGridProps = $props();
</script>

<IconTileGrid
    centerLabels
    tiles={recipes.map(({ recipe_id, icon, name }) => ({
        iconUrl: icon,
        label: name,
        onclick: () => onRecipeClick(recipe_id),
    }))}
/>

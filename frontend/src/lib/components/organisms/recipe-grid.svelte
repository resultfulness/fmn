<script lang="ts">
import IconTileGrid from "$lib/components/molecules/icon-tile-grid.svelte";
import type { Recipe } from "$lib/schemas/recipes";

interface RecipeGridProps {
    recipes: Recipe[];
    addRecipe: (recipe_id: number) => void;
}

const { recipes, addRecipe }: RecipeGridProps = $props();
</script>

<IconTileGrid
    centerLabels
    tiles={recipes.map(({ recipe_id, icon, name }) => ({
        iconUrl: icon,
        label: name,
        onclick: () => addRecipe(recipe_id),
    }))}
/>

<script lang="ts">
import IconTile from "$lib/ui/molecules/icon-tile.svelte";
import type { Recipe } from "./recipe";

interface RecipeTileProps {
    recipe: Recipe;
    onclick: (id: number) => void;
}

const { recipe, onclick }: RecipeTileProps = $props();
</script>

<IconTile
    centerLabel
    iconUrl={recipe.icon}
    label={recipe.name}
    onclick={() => onclick(recipe.recipe_id)}
/>

<script lang="ts">
import AnchorItem from "$lib/ui/molecules/anchor-item.svelte";
import RecipeWide from "./recipe-wide.svelte";
import type { Recipe } from "./recipe";

const { recipes }: { recipes: Recipe[] } = $props();
</script>

<ul class="item-list">
    {#each recipes as recipe}
        <AnchorItem
            href={`/recipes/${recipe.recipe_id}`}
            ariaLabel={`edit recipe ${recipe.name}`}
        >
            <RecipeWide {recipe} />
        </AnchorItem>
    {/each}
</ul>

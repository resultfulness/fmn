<script lang="ts">
import type { Recipe } from "$lib/schemas/recipes";
import ListItemLink from "$lib/components/molecules/list-item-link.svelte";
import RecipeListItem from "./recipe-list-item.svelte";

const { recipes }: { recipes?: Recipe[] } = $props();
</script>

{#if recipes === undefined}
    loading...
{:else if recipes.length <= 0}
    no recipes here
{:else}
    <ul>
        {#each recipes as recipe}
            <ListItemLink
                href={`/recipes/${recipe.recipe_id}`}
                ariaLabel={`edit recipe ${recipe.name}`}
            >
                <RecipeListItem {recipe} />
            </ListItemLink>
        {/each}
    </ul>
{/if}

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

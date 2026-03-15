<script lang="ts">
import type { Item } from "$lib/schemas/items";
import Ingredient from "./ingredient.svelte";

interface RecipeIngredientListProps {
    ingredients: { item: Item; quantity: number }[];
}

const { ingredients }: RecipeIngredientListProps = $props();
</script>

<ul>
    {#each ingredients as { item, quantity }}
        <Ingredient {item} {quantity} displayOnly />
    {/each}
</ul>

<style>
ul {
    padding: 0;
    list-style-type: none;
    margin: 0;
    display: grid;
    gap: 0.25rem;
    border-radius: calc(var(--rounding) / 2);
    overflow: hidden;
}
</style>

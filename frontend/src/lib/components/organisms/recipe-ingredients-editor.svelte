<script lang="ts">
import type { Item } from "$lib/schemas/items";
import type { RecipeItem } from "$lib/schemas/recipes";
import Search from "../molecules/search.svelte";
import Ingredient from "./ingredient.svelte";
import ItemList from "./item-list.svelte";

interface RecipeIngredientsEditorProps {
    recipeItems?: RecipeItem[];
    items?: Item[];
}

const { recipeItems = $bindable(), items }: RecipeIngredientsEditorProps =
    $props();

let searchterm = $state("");

function getItemDetail(id: number) {
    return items?.find(({ item_id }) => item_id === id);
}

function addToRecipe(id: number) {
    recipeItems?.push({ item_id: id, quantity: 1 });
}
</script>

<div>
    <span>Ingredients</span>
    <ul>
        {#if recipeItems === undefined}
            loading...
        {:else if recipeItems.length <= 0}
            no ingredients
        {:else}
            {#each recipeItems as { quantity, item_id }}
                {@const item = getItemDetail(item_id)}
                <Ingredient {item} {quantity} />
            {/each}
        {/if}
    </ul>
    <div class="add-ingredients">
        <span>Add ingredients</span>
        <Search bind:searchterm placeholder="search for ingredients..." />
        <ItemList
            items={items
                ?.filter(
                    ({ item_id }) =>
                        !recipeItems?.some(item => item.item_id === item_id)
                )
                .filter(item => item.name.includes(searchterm))}
            onclick={id => addToRecipe(id)}
        />
    </div>
</div>

<style>
.add-ingredients {
    display: grid;
    gap: 1rem;
}

ul {
    padding: 0;
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
}
</style>

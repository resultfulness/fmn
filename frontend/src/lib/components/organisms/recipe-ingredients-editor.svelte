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

let { recipeItems = $bindable(), items }: RecipeIngredientsEditorProps =
    $props();

let searchterm = $state("");
let itemsFiltered = $derived(
    items?.filter(
        item =>
            item.name.includes(searchterm) &&
            !recipeItems?.some(
                recipeItem => recipeItem.item_id === item.item_id
            )
    )
);
let expanded = $state(-1);

function getItemDetail(id: number) {
    return items?.find(({ item_id }) => item_id === id);
}

function addToRecipe(id: number) {
    recipeItems?.push({ item_id: id, quantity: 1 });
}

function removeFromRecipe(id: number) {
    recipeItems = recipeItems?.filter(({ item_id }) => item_id !== id);
    expanded = -1;
}
</script>

<div>
    <span>Ingredients</span>
    {#if recipeItems === undefined}
        loading...
    {:else if recipeItems.length <= 0}
        no ingredients
    {:else}
        <ul>
            {#each recipeItems as { item_id }, i}
                {@const item = getItemDetail(item_id)}
                <Ingredient
                    {item}
                    bind:quantity={recipeItems[i].quantity}
                    expanded={expanded === i}
                    onexpand={() => (expanded = i)}
                    ondelete={() => removeFromRecipe(item_id)}
                />
            {/each}
        </ul>
    {/if}
    <div class="add-ingredients">
        <span>Add ingredients</span>
        <Search bind:searchterm placeholder="search for ingredients..." />
        <ItemList items={itemsFiltered} onclick={id => addToRecipe(id)} />
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

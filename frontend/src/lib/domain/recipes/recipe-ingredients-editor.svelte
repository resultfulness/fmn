<script lang="ts">
import type { RecipeItem } from "./recipes";
import type { Item } from "../items/items";
import Ingredient from "./ingredient.svelte";
import Search from "$lib/ui/molecules/search.svelte";
import ItemList from "../items/item-list.svelte";

interface RecipeIngredientsEditorProps {
    recipeItems?: RecipeItem[];
    items: Item[];
}

let { recipeItems = $bindable(), items }: RecipeIngredientsEditorProps =
    $props();

let searchterm = $state("");
let itemsFiltered = $derived(
    items.filter(
        item =>
            item.name.includes(searchterm) &&
            !recipeItems?.some(
                recipeItem => recipeItem.item_id === item.item_id
            )
    )
);
let expanded = $state(-1);

function addToRecipe(id: number) {
    recipeItems?.push({ item_id: id, quantity: 1 });
    expanded = recipeItems ? recipeItems.length - 1 : -1;
}

function removeFromRecipe(id: number) {
    recipeItems = recipeItems?.filter(({ item_id }) => item_id !== id);
    expanded = -1;
}
</script>

<div>
    <span>Ingredients</span>
    {#if recipeItems && recipeItems.length > 0}
        <ul>
            {#each recipeItems as { item_id }, i}
                {@const item = items.find(item => item.item_id === item_id)}
                <Ingredient
                    {item}
                    bind:quantity={recipeItems[i].quantity}
                    expanded={expanded === i}
                    onexpand={() => (expanded = i)}
                    ondelete={() => removeFromRecipe(item_id)}
                />
            {/each}
        </ul>
    {:else}
        <div class="text-subtitle text-center" style:margin-block="1rem">
            no ingredients
        </div>
    {/if}
    <div class="add-ingredients">
        <span>Add ingredients</span>
        <Search bind:searchterm placeholder="search for ingredients..." />
        {#if itemsFiltered.length > 0}
            <ItemList items={itemsFiltered} onclick={id => addToRecipe(id)} />
        {:else}
            <div class="text-subtitle text-center">
                no items matching {searchterm}
            </div>
        {/if}
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

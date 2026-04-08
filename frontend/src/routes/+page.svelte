<script lang="ts">
import api from "$lib/api";
import Details from "$lib/components/atoms/details.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import Search from "$lib/components/molecules/search.svelte";
import CartGrid from "$lib/components/organisms/cart-grid.svelte";
import EditItem, {
    showItemEdit,
} from "$lib/components/organisms/edit-item.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemGrid from "$lib/components/organisms/item-grid.svelte";
import RecipeGrid from "$lib/components/organisms/recipe-grid.svelte";
import ListPage from "$lib/components/templates/list-page.svelte";
import type { CartItem } from "$lib/schemas/cart";
import type { Item } from "$lib/schemas/items";
import { PencilLine, Redo, Undo, X } from "@lucide/svelte";
import type { Recipe } from "$lib/schemas/recipes";
import { onMount } from "svelte";

let cart = $state<CartItem[]>();
let items = $state<Item[]>();
let recipes = $state<Recipe[]>();
let searchterm = $state("");
let cartmode = $state<"edit" | "use">("use");

let itemsFiltered = $derived(
    items?.filter(
        item =>
            item.name.includes(searchterm) &&
            !cart?.some(cartItem => cartItem.item_id === item.item_id)
    )
);

let recipesFiltered = $derived(
    recipes?.filter(recipe => recipe.name.includes(searchterm))
);

const resetCart = (_cart: CartItem[]) => (cart = _cart);

onMount(() => {
    api.cart.readAll().then(resetCart);
    api.items.readAll().then(_items => (items = _items));
    api.recipes.readAll().then(_recipes => (recipes = _recipes));
});

const removeItem = (id: number) => api.cart.removeItem(id).then(resetCart);
const addItem = (id: number) => api.cart.addItem(id).then(resetCart);
const addRecipe = (id: number) => api.cart.addRecipe(id).then(resetCart);

let clickItem = $derived(
    cartmode === "use"
        ? removeItem
        : (itemId: number) => {
              let cartItem = cart?.find(v => v.item_id === itemId)!;
              showItemEdit(items?.find(v => v.item_id === itemId)!, cartItem)
                  .then((quantity: number | null) =>
                      api.cart.updateItem(itemId, {
                          quantity: quantity ?? cartItem.quantity,
                          description: cartItem.description,
                      })
                  )
                  .then(resetCart);
          }
);

const undo = () => api.cart.undo().then(resetCart);
const redo = () => api.cart.redo().then(resetCart);
const clear = () => api.cart.clear().then(() => (cart = []));

HeaderState.title = "shopping";
delete HeaderState.backUrl;
</script>

<ListPage>
    <EditItem />
    {#if items && cart && cart.length > 0}
        <CartGrid {cart} {items} {clickItem} />
    {:else}
        <div class="text-subtitle text-center" style:margin-block="3rem">
            cart empty!
        </div>
    {/if}
    <div class="grid-separator">
        <h2 class="text-heading">{cartmode === "use" ? "Add" : "Edit"}</h2>
        <IconButton
            variant="secondary"
            icon={cartmode === "use" ? PencilLine : X}
            onclick={() => {
                cartmode = cartmode === "use" ? "edit" : "use";
            }}
        />
    </div>
    {#if cartmode === "use"}
        <Details summary="Recipes" open>
            {#if recipesFiltered && recipesFiltered.length > 0}
                <RecipeGrid recipes={recipesFiltered} {addRecipe} />
            {:else if searchterm}
                <div class="text-subtitle text-center">
                    no recipes matching {searchterm}
                </div>
            {/if}
        </Details>
        <Details summary="Items" open>
            {#if itemsFiltered && itemsFiltered.length > 0}
                <ItemGrid items={itemsFiltered} {addItem} />
            {:else if searchterm}
                <div class="text-subtitle text-center">
                    no items matching {searchterm}
                </div>
            {/if}
        </Details>
    {/if}
</ListPage>
<FooterExtension>
    <IconButton variant="secondary" icon={Undo} onclick={undo} />
    <IconButton variant="secondary" icon={Redo} onclick={redo} />
    <Search bind:searchterm placeholder="search for stuff..." />
</FooterExtension>

<style>
.grid-separator {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
</style>

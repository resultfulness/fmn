<script lang="ts">
import api from "$lib/api";
import {
    cartItemDisplays,
    type CartItem,
    type CartItemUpdate,
} from "$lib/domain/cart/cart";
import CartGrid from "$lib/domain/cart/cart-grid.svelte";
import CartItemEditForm from "$lib/domain/cart/cart-item-edit-form.svelte";
import ItemGrid from "$lib/domain/items/item-grid.svelte";
import type { Item } from "$lib/domain/items/item";
import RecipeGrid from "$lib/domain/recipes/recipe-grid.svelte";
import type { Recipe } from "$lib/domain/recipes/recipe";
import Details from "$lib/ui/elements/details.svelte";
import Dialog from "$lib/ui/elements/dialog.svelte";
import { HeaderState } from "$lib/ui/header.svelte";
import FooterExtension from "$lib/ui/molecules/footer-extension.svelte";
import IconButton from "$lib/ui/molecules/icon-button.svelte";
import Search from "$lib/ui/molecules/search.svelte";
import ListPage from "$lib/ui/templates/list-page.svelte";
import { pushToast } from "$lib/ui/toast.svelte";
import { PencilLine, Redo, Undo, X } from "@lucide/svelte";
import { onMount } from "svelte";

let cart: CartItem[] | undefined = $state();
let items: Item[] | undefined = $state();
let recipes: Recipe[] | undefined = $state();
let searchterm = $state("");
let cartmode: "editing" | "shopping" = $state("shopping");
let cartItemEditDialog: HTMLDialogElement = $state()!;
let cartItemEditId: number | undefined = $state();
let cartItemEditItem: Item | undefined = $state();
let cartItemEditCartItem: CartItem | undefined = $state();

const itemFound = (item: Item) =>
    item.name.toLowerCase().includes(searchterm.toLowerCase());
const itemInCart = (item: Item) =>
    cart?.some(cartItem => cartItem.item_id === item.item_id);

let itemsFiltered = $derived(
    items?.filter(item => !itemInCart(item) && itemFound(item))
);

const recipeFound = (recipe: Recipe) =>
    recipe.name.toLowerCase().includes(searchterm.toLowerCase());
let recipesFiltered = $derived(recipes?.filter(recipeFound));

const setCart = (new_cart: CartItem[]) => (cart = new_cart);
// const clearCart = () => api.cart.clear().then(() => (cart = []));
const undo = () => api.cart.undo().then(setCart);
const redo = () => api.cart.redo().then(setCart);

const cartRemoveItem = (id: number) => api.cart.removeItem(id).then(setCart);
const cartAddItem = (id: number) =>
    api.cart
        .addItem(id)
        .then(setCart)
        .then(() => (searchterm = ""));
const cartUpdateItem = (id: number, newItem: CartItemUpdate) =>
    api.cart.updateItem(id, newItem).then(setCart);

const cartAddRecipe = (id: number) =>
    api.cart
        .addRecipe(id)
        .then(setCart)
        .then(() => (searchterm = ""));

const showCartItemEdit = (id: number) => {
    cartItemEditId = id;
    cartItemEditItem = items?.find(v => v.item_id === id);
    cartItemEditCartItem = $state.snapshot(cart?.find(v => v.item_id === id));
    cartItemEditDialog.showModal();
};

let onCartItemClick = $derived(
    cartmode === "shopping" ? cartRemoveItem : showCartItemEdit
);

const handleCartUpdateItem = (e: SubmitEvent) => {
    e.preventDefault();
    if (!cartItemEditId || !cartItemEditCartItem) return;
    cartUpdateItem(cartItemEditId, cartItemEditCartItem)
        .then(() => pushToast("item saved", "success"))
        .then(() => cartItemEditDialog.close());
};

onMount(() => {
    HeaderState.title = "shopping";
    delete HeaderState.backUrl;

    api.cart.readAll().then(setCart);
    api.items.readAll().then(_items => (items = _items));
    api.recipes.readAll().then(_recipes => (recipes = _recipes));
});
</script>

<ListPage>
    <Dialog title="Editing cart item" bind:dialog={cartItemEditDialog}>
        {#if cartItemEditItem && cartItemEditCartItem}
            <CartItemEditForm
                onsubmit={handleCartUpdateItem}
                item={cartItemEditItem}
                bind:cartItem={cartItemEditCartItem}
            />
        {/if}
    </Dialog>
    {#if items && cart && cart.length > 0}
        <CartGrid cartItems={cartItemDisplays(items, cart)} {onCartItemClick} />
    {:else}
        <div class="text-subtitle text-center" style:margin-block="3rem">
            cart empty!
        </div>
    {/if}
    <div class="grid-separator">
        <h2 class="text-heading">{cartmode === "shopping" ? "Add" : "Edit"}</h2>
        <IconButton
            variant="secondary"
            icon={cartmode === "shopping" ? PencilLine : X}
            onclick={() => {
                cartmode = cartmode === "shopping" ? "editing" : "shopping";
            }}
        />
    </div>
    <div class="add">
        {#if cartmode === "shopping"}
            <Details summary="Recipes" open>
                {#if recipesFiltered && recipesFiltered.length > 0}
                    <RecipeGrid
                        recipes={recipesFiltered}
                        onRecipeClick={cartAddRecipe}
                    />
                {:else if searchterm}
                    <div class="text-subtitle text-center">
                        no recipes matching {searchterm}
                    </div>
                {/if}
            </Details>
            <Details summary="Items" open>
                {#if itemsFiltered && itemsFiltered.length > 0}
                    <ItemGrid items={itemsFiltered} onItemClick={cartAddItem} />
                {:else if searchterm}
                    <div class="text-subtitle text-center">
                        no items matching {searchterm}
                    </div>
                {/if}
            </Details>
        {/if}
    </div>
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

.add {
    display: grid;
    gap: 1rem;
}
</style>

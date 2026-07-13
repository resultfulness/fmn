<script lang="ts">
import { cartItemDisplays, type CartItem } from "$lib/domain/cart/cart";
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
import { pushToast } from "$lib/ui/toast";
import { PencilLine, Redo, Undo, X } from "@lucide/svelte";
import { onMount } from "svelte";
import itemStore from "$lib/domain/items/store.svelte";
import recipeStore from "$lib/domain/recipes/store.svelte";
import cartStore from "$lib/domain/cart/store";

let searchterm = $state("");
let cartmode: "editing" | "shopping" = $state("shopping");
let cartItemEditDialog: HTMLDialogElement = $state()!;
let cartItemEditId: number | undefined = $state();
let cartItemEditItem: Item | undefined = $state();
let cartItemEditCartItem: CartItem | undefined = $state();

const itemFound = (item: Item) =>
    item.name.toLowerCase().includes(searchterm.toLowerCase());
const itemInCart = (item: Item) =>
    $cartStore.some(cartItem => cartItem.item_id === item.item_id);

let itemsFiltered = $derived(
    itemStore.items.filter(item => !itemInCart(item) && itemFound(item))
);

const recipeFound = (recipe: Recipe) =>
    recipe.name.toLowerCase().includes(searchterm.toLowerCase());
let recipesFiltered = $derived(recipeStore.recipes.filter(recipeFound));

function showCartItemEdit(id: number) {
    cartItemEditId = id;
    cartItemEditItem = itemStore.items.find(v => v.item_id === id);
    cartItemEditCartItem = $state.snapshot(
        $cartStore.find(v => v.item_id === id)
    );
    cartItemEditDialog.showModal();
}

let onCartItemClick = $derived(
    cartmode === "shopping" ? cartStore.removeItem : showCartItemEdit
);

async function handleCartUpdateItem(e: SubmitEvent) {
    e.preventDefault();
    if (!cartItemEditId || !cartItemEditCartItem) return;
    await cartStore.updateItem(cartItemEditId, cartItemEditCartItem);
    pushToast("item saved", "success");
    cartItemEditDialog.close();
}

const clearSearch = () => (searchterm = "");

onMount(() => {
    HeaderState.title = "shopping";
    delete HeaderState.backUrl;

    cartStore.load();
    itemStore.load();
    recipeStore.load();
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
    {#if $cartStore.length > 0}
        <CartGrid
            cartItems={cartItemDisplays(itemStore.items, $cartStore)}
            {onCartItemClick}
        />
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
                {#if recipesFiltered.length > 0}
                    <RecipeGrid
                        recipes={recipesFiltered}
                        onRecipeClick={(recipe_id: number) => {
                            cartStore.addRecipe(recipe_id).then(clearSearch);
                        }}
                    />
                {:else if searchterm}
                    <div class="text-subtitle text-center">
                        no recipes matching {searchterm}
                    </div>
                {/if}
            </Details>
            <Details summary="Items" open>
                {#if itemsFiltered.length > 0}
                    <ItemGrid
                        items={itemsFiltered}
                        onItemClick={(item_id: number) => {
                            cartStore.addItem(item_id).then(clearSearch);
                        }}
                    />
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
    <IconButton variant="secondary" icon={Undo} onclick={cartStore.undo} />
    <IconButton variant="secondary" icon={Redo} onclick={cartStore.redo} />
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

<script lang="ts">
import api from "$lib/api";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import Search from "$lib/components/molecules/search.svelte";
import CartGrid from "$lib/components/organisms/cart-grid.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemGrid from "$lib/components/organisms/item-grid.svelte";
import ListPage from "$lib/components/templates/list-page.svelte";
import type { CartItem } from "$lib/schemas/cart";
import type { Item } from "$lib/schemas/items";
import { ListFilter, Redo, Undo } from "@lucide/svelte";
import { onMount } from "svelte";

let cart = $state<CartItem[]>();
let items = $state<Item[]>();
let searchterm = $state("");

let itemsFiltered = $derived(
    items?.filter(
        item =>
            item.name.includes(searchterm) &&
            !cart?.some(({ item_id }) => item.item_id === item_id)
    )
);

const resetCart = (_cart: CartItem[]) => (cart = _cart);

onMount(() => {
    api.cart.readAll().then(resetCart);
    api.items.readAll().then(_items => (items = _items));
});

const removeItem = (id: number) => api.cart.removeItem(id).then(resetCart);
const addItem = (id: number) => api.cart.addItem(id).then(resetCart);

const undo = () => api.cart.undo().then(resetCart);
const redo = () => api.cart.redo().then(resetCart);

HeaderState.title = "shopping";
delete HeaderState.backUrl;
</script>

<ListPage>
    <CartGrid {cart} {items} {removeItem} />
    <div class="grid-separator">
        <h2 class="text-heading">Add</h2>
        <IconButton variant="secondary" icon={ListFilter} />
    </div>
    <ItemGrid items={itemsFiltered} {addItem} />
</ListPage>
<FooterExtension>
    <IconButton variant="secondary" icon={Undo} onclick={undo} />
    <IconButton variant="secondary" icon={Redo} onclick={redo} />
    <Search bind:searchterm placeholder="search for items..." />
</FooterExtension>

<style>
.grid-separator {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
</style>

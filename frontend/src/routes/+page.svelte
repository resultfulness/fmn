<script lang="ts">
import api from "$lib/api";
import CartGrid from "$lib/components/organisms/cart-grid.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemGrid from "$lib/components/organisms/item-grid.svelte";
import type { CartItem } from "$lib/schemas/cart";
import type { ItemShort } from "$lib/schemas/items";
import { onMount } from "svelte";

let cart = $state<CartItem[]>();
let items = $state<ItemShort[]>();

onMount(() => {
    api.cart
        .readAll()
        .then(_cart => (cart = _cart))
        .catch(e => alert(e));

    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => alert(e));
});

HeaderState.title = "shopping";
delete HeaderState.backUrl;
</script>

<div class="page">
    <CartGrid items={cart} />
    <h2 class="text-heading">Add</h2>
    <ItemGrid {items} />
</div>

<style>
.page {
    padding: 1rem;
    overflow-y: auto;
}
</style>

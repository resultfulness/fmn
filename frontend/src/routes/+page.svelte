<script lang="ts">
import api from "$lib/api";
import CartGrid from "$lib/components/organisms/cart-grid.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import type { CartItem } from "$lib/schemas/cart";
import { Unit } from "$lib/schemas/items";
import { onMount } from "svelte";

HeaderState.title = "shopping";

let items = $state<CartItem[]>();

onMount(() => {
    api.cart
        .readAll()
        .then(_items => (items = _items))
        .catch(e => alert(e));
});
</script>

<div class="page">
    <CartGrid {items} />
</div>

<style>
.page {
    padding: 1rem;
    overflow-y: auto;
}
</style>

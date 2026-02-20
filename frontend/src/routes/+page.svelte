<script lang="ts">
import api from "$lib/api";
import CartGrid from "$lib/components/organisms/cart-grid.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import type { CartItem } from "$lib/schemas/cart";
import { onMount } from "svelte";

let cart = $state<CartItem[]>();

onMount(() => {
    api.cart
        .readAll()
        .then(_cart => (cart = _cart))
        .catch(e => alert(e));
});

HeaderState.title = "shopping";
delete HeaderState.backUrl;
</script>

<div class="page">
    <CartGrid items={cart} />
</div>

<style>
.page {
    padding: 1rem;
    overflow-y: auto;
}
</style>

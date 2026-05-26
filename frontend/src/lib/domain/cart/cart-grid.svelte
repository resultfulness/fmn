<script lang="ts">
import type { CartItemDisplay } from "./cart";
import CartItemTile from "./cart-item-tile.svelte";

interface CartGridProps {
    cartItems: CartItemDisplay[],
    onCartItemClick: (item_id: number) => void;
}

const { cartItems, onCartItemClick }: CartGridProps = $props();
</script>

<div class="tile-grid">
    {#each cartItems as cartItem}
        <CartItemTile {cartItem} onclick={onCartItemClick} />
    {/each}
</div>

<script lang="ts">
import type { CartItem } from "$lib/schemas/cart";
import CartItemTile from "./cart-item-tile.svelte";

const { items }: { items: CartItem[] | undefined } = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    cart is empty
{:else}
    <div class="cart-grid">
        {#each items as item}
            <CartItemTile {item} />
        {/each}
    </div>
{/if}

<style>
.cart-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.5rem;
    margin-inline: auto;
}
</style>

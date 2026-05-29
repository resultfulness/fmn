<script lang="ts">
import type { EventHandler } from "svelte/elements";
import type { Item } from "../items/item";
import { cartItemDisplay, type CartItem } from "./cart";
import Form from "$lib/ui/elements/form.svelte";
import InputField from "$lib/ui/molecules/input-field.svelte";
import Button from "$lib/ui/elements/button.svelte";
import CartItemTile from "./cart-item-tile.svelte";

interface CartItemEditFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    item: Item;
    cartItem: CartItem;
}

let {
    onsubmit,
    item,
    cartItem = $bindable(),
}: CartItemEditFormProps = $props();
</script>

<Form {onsubmit}>
    <span class="item-tile">
        <CartItemTile
            onclick={() => {}}
            cartItem={cartItemDisplay(item, cartItem)}
        />
    </span>
    <InputField
        type="number"
        name="quantity"
        bind:value={cartItem.quantity}
        label="Quantity"
        placeholder="enter quantity..."
    />
    {#snippet actions()}
        <Button>save</Button>
    {/snippet}
</Form>

<style>
.item-tile {
    display: grid;
    grid-template-columns: 128px;
    justify-content: center;
}
</style>

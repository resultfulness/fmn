<script module lang="ts">
import api from "$lib/api";
import Button from "$lib/components/atoms/button.svelte";
import { CartItem } from "$lib/schemas/cart";
import type { Item } from "$lib/schemas/items";
import InputField from "../molecules/input-field.svelte";

let dialog: HTMLDialogElement = $state()!;
let dialogTitle: string = $state("");
let dialogMessage: string = $state("");
let save: (() => void) | undefined = $state();
let cancel: (() => void) | undefined = $state();
let quantity: string = $state("");
let endText: string = $state("");

export function showItemEdit(
    item: Item,
    cartItem: CartItem
): Promise<number | null> {
    dialogTitle = item.name;
    endText = item.unit;
    quantity = cartItem.quantity?.toString() ?? "";
    dialog.showModal();
    return new Promise(res => {
        save = () => res(+quantity);
        cancel = () => res(null);
    });
}
</script>

<dialog class="confirmation-dialog" bind:this={dialog} closedby="any">
    <h2 class="text-header text-center">{dialogTitle}</h2>
    <p class="message text-content text-center">{dialogMessage}</p>
    <InputField label="quantity" bind:value={quantity} {endText} />
    <div class="submit">
        <form method="dialog" onsubmit={cancel}>
            <Button variant="secondary">cancel</Button>
        </form>
        <form method="dialog" onsubmit={save}>
            <Button variant="danger">save</Button>
        </form>
    </div>
</dialog>

<style>
.confirmation-dialog {
    position: fixed;
    border: 0;
    inset: 0;
    border-radius: var(--rounding);
    background-color: var(--clr-base);
    color: var(--clr-text);
    box-shadow: var(--shadow);
    padding: 1rem;
    max-width: 33ch;
    gap: 1rem;

    &[open] {
        display: grid;
    }

    &::backdrop {
        background-color: rgba(0 0 0 / 0.5);
    }

    h2 {
        margin: 0;
    }

    .message {
        margin: 0;
    }

    .submit {
        display: flex;
        gap: 1rem;
    }

    .submit > form {
        flex: 1;
        display: grid;
    }
}
</style>

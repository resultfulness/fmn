<script module lang="ts">
import Button from "$lib/components/atoms/button.svelte";

let dialog: HTMLDialogElement = $state()!;
let dialogTitle: string = $state("");
let dialogMessage: string = $state("");
let confirm: (() => void) | undefined = $state();
let cancel: (() => void) | undefined = $state();

export function showConfirmationDialog(
    title: string,
    message: string
): Promise<boolean> {
    dialogTitle = title;
    dialogMessage = message;
    dialog.showModal();
    return new Promise(res => {
        confirm = () => res(true);
        cancel = () => res(false);
    });
}
</script>

<dialog class="confirmation-dialog" bind:this={dialog} closedby="any">
    <h2 class="title">{dialogTitle}</h2>
    <p class="subtitle">{dialogMessage}</p>
    <div class="submit">
        <form method="dialog" onsubmit={cancel}>
            <Button variant="secondary">cancel</Button>
        </form>
        <form method="dialog" onsubmit={confirm}>
            <Button variant="danger">confirm</Button>
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
}

.confirmation-dialog[open] {
    display: grid;
}

.confirmation-dialog::backdrop {
    background-color: rgba(0 0 0 / 0.5);
}

.confirmation-dialog .title {
    text-align: center;
    font: var(--font-heading);
    margin: 0;
}

.confirmation-dialog .subtitle {
    text-align: center;
    margin: 0;
}

.confirmation-dialog .submit {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr 1fr;
}

.confirmation-dialog .submit > * {
    display: grid;
}
</style>

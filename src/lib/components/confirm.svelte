<script module lang="ts">
import Button from "./button.svelte";

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

<dialog class="confirmation-dialog" bind:this={dialog}>
    <h2 class="confirmation-dialog-title">{dialogTitle}</h2>
    <p class="confirmation-dialog-subtitle">{dialogMessage}</p>
    <div class="confirmation-dialog-submit">
        <form method="dialog" onsubmit={cancel}>
            <Button variant="secondary">cancel</Button>
        </form>
        <form method="dialog" onsubmit={confirm}>
            <Button>confirm</Button>
        </form>
    </div>
</dialog>

<style>
.confirmation-dialog {
    position: fixed;
    border: 0;
    inset: 0;
    border-radius: 0.5rem;
    background-color: var(--clr-s1);
    color: var(--clr-text);
    box-shadow: 0 1px 4px 0 rgb(0 0 0 / 0.1);
    padding: 1rem;
    max-width: 33ch;
}

.confirmation-dialog::backdrop {
    background-color: rgba(0 0 0 / 0.5);
}

.confirmation-dialog-title {
    text-align: center;
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
    margin-bottom: 0.5rem;
}

.confirmation-dialog-subtitle {
    text-align: center;
    margin: 0;
    margin-bottom: 1rem;
}

.confirmation-dialog-submit {
    display: grid;
    gap: 0.5rem;
    grid-template-columns: 1fr 1fr;
}

.confirmation-dialog-submit > * {
    display: grid;
}
</style>

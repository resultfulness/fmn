<script lang="ts">
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";

interface FormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    children: Snippet;
    actions: Snippet;
}

const { onsubmit, children, actions }: FormProps = $props();
</script>

<form {onsubmit}>
    {@render children()}
    <div class="actions">{@render actions()}</div>
</form>

<style>
form {
    display: grid;
    gap: 1rem;

    .actions {
        padding-block: 1rem;
        display: flex;
        gap: 1rem;
        position: sticky;
        bottom: 0;
        background-color: var(--clr-base);

        :global(> *) {
            flex: 1;
        }
    }
}
</style>

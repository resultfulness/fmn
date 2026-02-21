<script lang="ts">
import { type ItemUpdate, unitPretty, UNITS } from "$lib/schemas/items";
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import DropdownField from "$lib/components/molecules/dropdown-field.svelte";
import InputField from "$lib/components/molecules/input-field.svelte";

interface ItemFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    item: ItemUpdate;
}

let { onsubmit, actions, item = $bindable() }: ItemFormProps = $props();
</script>

<form {onsubmit}>
    <InputField
        type="text"
        name="name"
        bind:value={item.name}
        label="Name"
        placeholder="enter name..."
    />
    <InputField
        type="text"
        name="icon"
        bind:value={item.icon}
        label="Icon URL"
        placeholder="enter url..."
    />
    <DropdownField
        options={[...UNITS]}
        labels={UNITS.map(unitPretty)}
        bind:value={item.unit}
        label="Unit"
        placeholder="pick a unit..."
        required
    />
    <div class="actions">{@render actions()}</div>
</form>

<style>
form {
    display: grid;
    gap: 1rem;

    .actions {
        padding-top: 1rem;
        display: flex;
        gap: 1rem;

        :global(> *) {
            flex: 1;
        }
    }
}
</style>

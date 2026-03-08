<script lang="ts">
import { type ItemUpdate, unitPretty, UNITS } from "$lib/schemas/items";
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import Form from "$lib/components/atoms/form.svelte";
import DropdownField from "$lib/components/molecules/dropdown-field.svelte";
import InputField from "$lib/components/molecules/input-field.svelte";

interface ItemFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    item: ItemUpdate;
}

let { onsubmit, actions, item = $bindable() }: ItemFormProps = $props();
</script>

<Form {onsubmit} {actions}>
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
</Form>

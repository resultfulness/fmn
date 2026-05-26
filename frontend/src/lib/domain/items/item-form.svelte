<script lang="ts">
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import type { ItemCreate, ItemUpdate } from "./items";
import Form from "$lib/ui/elements/form.svelte";
import InputField from "$lib/ui/molecules/input-field.svelte";
import SelectField from "$lib/ui/molecules/select-field.svelte";
import { ALL_UNITS, unitPretty } from "./units";

interface ItemFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    item: ItemUpdate | Partial<ItemCreate>;
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
    <SelectField
        options={[...ALL_UNITS]}
        labels={ALL_UNITS.map(unitPretty)}
        bind:value={item.unit}
        label="Unit"
        placeholder="pick a unit..."
        required
    />
</Form>

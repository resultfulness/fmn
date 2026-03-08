<script lang="ts">
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import type { RecipeCreate } from "$lib/schemas/recipes";
import Form from "$lib/components/atoms/form.svelte";
import InputField from "$lib/components/molecules/input-field.svelte";
import TextareaField from "$lib/components/molecules/textarea-field.svelte";

interface RecipeCreateFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    recipe: Partial<RecipeCreate>;
}

const {
    onsubmit,
    actions,
    recipe = $bindable(),
}: RecipeCreateFormProps = $props();
</script>

<Form {onsubmit} {actions}>
    <InputField
        type="text"
        name="name"
        bind:value={recipe.name}
        label="Name"
        placeholder="enter name..."
    />
    <InputField
        type="text"
        name="icon"
        bind:value={recipe.icon}
        label="Icon URL"
        placeholder="enter url..."
    />
    <InputField
        type="number"
        name="servings"
        bind:value={recipe.servings}
        label="Servings"
        placeholder="enter servings amount..."
    />
    <TextareaField
        label="Description"
        bind:value={recipe.description}
        placeholder="enter description..."
    />
</Form>

<script lang="ts">
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import type { RecipeUpdate } from "$lib/schemas/recipes";
import Form from "$lib/components/atoms/form.svelte";
import InputField from "$lib/components/molecules/input-field.svelte";
import TextareaField from "$lib/components/molecules/textarea-field.svelte";
import RecipeIngredientsEditor from "./recipe-ingredients-editor.svelte";
import type { Item } from "$lib/schemas/items";

interface RecipeUpdateFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    recipe: RecipeUpdate;
    items?: Item[];
}

const {
    onsubmit,
    actions,
    recipe = $bindable(),
    items,
}: RecipeUpdateFormProps = $props();
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
    <RecipeIngredientsEditor bind:recipeItems={recipe.items} {items} />
</Form>

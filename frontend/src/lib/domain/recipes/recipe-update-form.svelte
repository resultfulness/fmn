<script lang="ts">
import type { Snippet } from "svelte";
import type { EventHandler } from "svelte/elements";
import type { RecipeUpdate } from "./recipes";
import type { Item } from "../items/items";
import Form from "$lib/ui/elements/form.svelte";
import InputField from "$lib/ui/molecules/input-field.svelte";
import TextareaField from "$lib/ui/molecules/textarea-field.svelte";
import RecipeIngredientsEditor from "./recipe-ingredients-editor.svelte";

interface RecipeUpdateFormProps {
    onsubmit: EventHandler<SubmitEvent, HTMLFormElement>;
    actions: Snippet;
    recipe: RecipeUpdate;
    items: Item[];
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

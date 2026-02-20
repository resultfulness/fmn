<script lang="ts">
import type { RecipeShort } from "$lib/schemas/recipes";
import LinkedListEntry from "../molecules/linked-list-entry.svelte";

const { recipe }: { recipe: RecipeShort } = $props();
</script>

<LinkedListEntry
    href={`/recipes/edit/${recipe.recipe_id}`}
    ariaLabel={`edit recipe ${recipe.name}`}
>
    <div class="layout">
        <img class="icon" src={recipe.icon} alt={recipe.name + " icon"} />
        <div class="details">
            <span class="text-heading">{recipe.name}</span>
            <span class="text-subtitle">{recipe.servings} servings</span>
        </div>
    </div>
</LinkedListEntry>

<style>
.layout {
    display: grid;
    align-items: center;
    grid-template-columns: auto 1fr;
    gap: 0.5rem;
}

.icon {
    width: 64px;
    aspect-ratio: 1;
    object-fit: cover;
    border-radius: var(--rounding);
}

.details {
    display: grid;
    height: fit-content;
    align-self: center;
    gap: 0.25rem;
}
</style>

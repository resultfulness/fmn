<script lang="ts">
import type { RecipeShort } from "$lib/schemas/recipes";

const { recipe }: { recipe: RecipeShort } = $props();
</script>

<li class="recipe-list-entry">
    <a
        href={`/recipes/edit/${recipe.recipe_id}`}
        aria-label={`edit recipe ${recipe.name}`}
    >
        <img class="icon" src={recipe.icon} alt={recipe.name + " icon"} />
        <div class="details">
            <span class="text-label">{recipe.name}</span>
            <span class="text-subtitle">{recipe.servings} servings</span>
        </div>
    </a>
</li>

<style>
.recipe-list-entry {
    background-color: var(--clr-surface);
    border-radius: var(--rounding);

    a {
        padding: 0.5rem;
        text-decoration: none;
        color: unset;
        display: grid;
        align-items: center;
        grid-template-columns: auto 1fr;
        gap: 0.5rem;

        &:hover {
            cursor: pointer;
        }
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
}
</style>

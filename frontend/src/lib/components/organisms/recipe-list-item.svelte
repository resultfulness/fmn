<script lang="ts">
import type { RecipeShort } from "$lib/schemas/recipes";
import Image from "$lib/components/atoms/image.svelte";

const { recipe }: { recipe: RecipeShort } = $props();
</script>

<div class="layout">
    <Image src={recipe.icon} alt="" width={64} />
    <div class="details">
        <span class="text-heading name">{recipe.name}</span>
        <span class="text-subtitle">{recipe.servings} servings</span>
    </div>
</div>

<style>
.layout {
    display: grid;
    align-items: center;
    grid-template-columns: auto 1fr;
    gap: 0.5rem;
}

.name {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
}

.details {
    display: grid;
    height: fit-content;
    align-self: center;
    gap: 0.25rem;
}
</style>

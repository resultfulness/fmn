<script lang="ts">
import type { RecipeShort } from "$lib/schemas/recipes";
import RecipeListEntry from "./recipe-list-entry.svelte";

const { recipes }: { recipes?: RecipeShort[] } = $props();
</script>

{#if recipes === undefined}
    loading...
{:else if recipes.length <= 0}
    no recipes here
{:else}
    <ul>
        {#each recipes as recipe}
            <RecipeListEntry {recipe} />
        {/each}
    </ul>
{/if}

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

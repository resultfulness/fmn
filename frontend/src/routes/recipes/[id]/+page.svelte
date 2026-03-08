<script lang="ts">
import api from "$lib/api";
import { onMount } from "svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import FormPage from "$lib/components/templates/form-page.svelte";
import type { Item } from "$lib/schemas/items";
import { pushToast } from "$lib/components/toast.svelte";
import Ingredient from "$lib/components/organisms/ingredient.svelte";
import Textarea from "$lib/components/atoms/textarea.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import { Pencil } from "@lucide/svelte";

let { data } = $props();
let items: Item[] | undefined = $state();

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => pushToast(e, "error"));
});

function getItemDetail(id: number) {
    return items?.find(({ item_id }) => item_id === id);
}

HeaderState.title = "";
HeaderState.backUrl = "/recipes";
</script>

<FormPage icon={data.recipe.icon} title={data.recipe.name}>
    <div class="text-content text-center">{data.recipe.servings} servings</div>
    <h3 class="text-heading">Ingredients</h3>
    {#if data.recipe.items.length <= 0}
        <div class="text-subtitle">no ingredients</div>
    {:else}
        <ul>
            {#each data.recipe.items as { item_id, quantity }}
                {@const item = getItemDetail(item_id)}
                <Ingredient {item} {quantity} displayOnly />
            {/each}
        </ul>
    {/if}
    <h3 class="text-heading">Description</h3>
    <Textarea readonly value={data.recipe.description} />
    <div class="fab">
        <IconButton
            icon={Pencil}
            href={`/recipes/${data.recipe.recipe_id}/edit`}
        />
    </div>
</FormPage>

<style>
h3 {
    margin: 0;
}

ul {
    padding: 0;
    list-style-type: none;
    margin: 0;
    display: grid;
    gap: 0.25rem;
    border-radius: calc(var(--rounding) / 2);
    overflow: hidden;
}

.fab {
    position: absolute;
    width: 48px;
    height: 48px;
    bottom: 1rem;
    right: 1rem;
}
</style>

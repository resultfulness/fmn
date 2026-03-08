<script lang="ts">
import type { Item } from "$lib/schemas/items";
import { GripVertical } from "@lucide/svelte";
import Image from "../atoms/image.svelte";

interface IngredientProps {
    item?: Item;
    quantity: number;
    displayOnly?: boolean;
}

const { item, quantity, displayOnly }: IngredientProps = $props();
</script>

{#if item !== undefined}
    <div class="ingredient">
        <GripVertical color="var(--clr-text-dim)" />
        <Image src={item.icon} alt="" width={48} />
        <span class="name text-content">{item.name}</span>
        <span class="text-subtitle">{quantity} {item.unit}</span>
    </div>
{/if}

<style>
.ingredient {
    padding: 0.25rem 1rem 0.25rem 0.25rem;
    background-color: var(--clr-surface);
    border-radius: var(--rounding);
    box-shadow: var(--shadow);
    display: flex;
    align-items: center;

    .name {
        flex: 1;
        margin-left: 0.25rem;
    }
}
</style>

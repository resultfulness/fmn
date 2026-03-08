<script lang="ts">
import type { Item } from "$lib/schemas/items";
import { GripVertical, Trash } from "@lucide/svelte";
import Image from "$lib/components/atoms/image.svelte";
import IconButton from "../molecules/icon-button.svelte";
import Input from "../atoms/input.svelte";
import type { MouseEventHandler } from "svelte/elements";

interface IngredientProps {
    item?: Item;
    quantity: number;
    displayOnly?: boolean;
    ondelete?: MouseEventHandler<HTMLButtonElement> | null;
    onexpand?: () => void;
    expanded?: boolean;
}

let {
    item,
    quantity = $bindable(),
    displayOnly,
    ondelete,
    onexpand,
    expanded = $bindable(),
}: IngredientProps = $props();

function toggleExpanded() {
    expanded = !expanded;
    if (expanded && onexpand) {
        onexpand();
    }
}
</script>

{#if item !== undefined}
    {#if displayOnly}
        <li class="ingredient-display">
            <Image src={item.icon} alt="" width={24} />
            <span class="name text-content">{item.name}</span>
            <span class="text-subtitle">{quantity} {item.unit}</span>
        </li>
    {:else}
        <li class="ingredient-wrapper" class:expanded>
            <button class="ingredient" type="button" onclick={toggleExpanded}>
                <GripVertical color="var(--clr-text-dim)" />
                <Image src={item.icon} alt="" width={48} />
                <span class="name text-content">{item.name}</span>
                <span class="amount text-subtitle">
                    {quantity ? quantity : "?"}
                    {item.unit}
                </span>
            </button>
            <div class="edit-area">
                <div class="content">
                    <IconButton
                        icon={Trash}
                        variant="danger"
                        onclick={ondelete}
                    />
                    <Input
                        placeholder="edit quantity..."
                        type="number"
                        required
                        bind:value={quantity}
                        endText={item.unit}
                    />
                </div>
            </div>
        </li>
    {/if}
{/if}

<style>
.ingredient-wrapper {
    background-color: var(--clr-overlay);
    border-radius: var(--rounding);
    box-shadow: var(--shadow);
    display: grid;
    grid-template-rows: 1fr 0fr;
    overflow: hidden;

    --_show-duration: 400ms;

    transition: grid-template-rows var(--_show-duration);

    &.expanded {
        grid-template-rows: 1fr 1fr;

        .edit-area {
            visibility: visible;
        }

        .amount {
            opacity: 0;
        }
    }
}

.ingredient {
    padding: 0.25rem 1rem 0.25rem 0.25rem;
    background-color: var(--clr-surface);
    border-radius: var(--rounding);
    display: flex;
    align-items: center;
    width: 100%;
    text-align: left;
    border: none;
    cursor: pointer;

    &:focus-visible {
        outline: 2px solid var(--clr-primary);
        outline-offset: -2px;
    }
}

.edit-area {
    min-height: 0;
    transition: visibility var(--_show-duration);
    visibility: hidden;

    .content {
        padding: 0.5rem;
        gap: 0.5rem;
        display: flex;
        align-items: center;
    }
}

.ingredient-display {
    padding: 0.5rem;
    background-color: var(--clr-surface);
    display: flex;
    align-items: center;
}

.name {
    flex: 1;
    margin-left: 0.25rem;
}

.amount {
    opacity: 1;
    transition: opacity var(--_show-duration);
}
</style>

<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";

const { item }: { item: ItemShort } = $props();
</script>

<li class="item-list-entry">
    <a
        href={`/items/edit/${item.item_id}`}
        aria-label={`edit item ${item.name}`}
    >
        <img class="icon" src={item.icon} alt={item.name + " icon"} />
        <span class="label">{item.name}</span>
        <span class="unit">{item.unit}</span>
    </a>
</li>

<style>
.item-list-entry {
    background-color: var(--clr-surface);
    border-radius: var(--rounding);

    a {
        padding: 0.25rem 1rem 0.25rem 0.5rem;
        text-decoration: none;
        color: unset;
        display: flex;
        align-items: center;
        gap: 0.5rem;

        &:hover {
            cursor: pointer;
        }
    }

    .icon {
        width: 48px;
        aspect-ratio: 1;
        object-fit: cover;
        border-radius: var(--rounding);
    }

    .label {
        flex: 1;
    }

    .unit {
        color: var(--clr-text-dim);
    }
}
</style>

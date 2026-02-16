<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";

const { item }: { item: ItemShort } = $props();
</script>

<li class="item-list-entry">
    <img class="icon" src={item.icon} alt={item.name + " icon"} />
    <span class="label">{item.name}</span>
    <span class="unit">{item.unit}</span>
    <a
        href={`/items/edit/${item.item_id}`}
        aria-label={`edit item ${item.name}`}
        style="position: absolute; inset: 0;"
    ></a>
</li>

<style>
.item-list-entry {
    position: relative;
    padding: 0.25rem 1rem 0.25rem 0.5rem;
    display: flex;
    background-color: var(--clr-surface);
    border-radius: var(--rounding);
    align-items: center;
    gap: 0.5rem;
}

.item-list-entry:hover {
    cursor: pointer;
}

.item-list-entry .icon {
    max-width: 48px;
    aspect-ratio: 1;
    object-fit: cover;
}

.item-list-entry .label {
    flex: 1;
}

.item-list-entry .unit {
    color: var(--clr-text-dim);
}
</style>

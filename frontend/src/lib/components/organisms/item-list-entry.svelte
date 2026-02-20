<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import LinkedListEntry from "$lib/components/molecules/linked-list-entry.svelte";

const { item }: { item: ItemShort } = $props();
</script>

<LinkedListEntry
    href={`/items/edit/${item.item_id}`}
    ariaLabel={`edit item ${item.name}`}
>
    <div class="layout">
        <img class="icon" src={item.icon} alt={item.name + " icon"} />
        <span class="label">{item.name}</span>
        <span class="unit">{item.unit}</span>
    </div>
</LinkedListEntry>

<style>
.layout {
    display: flex;
    align-items: center;
    gap: 0.5rem;
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
</style>

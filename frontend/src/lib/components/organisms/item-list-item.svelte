<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";

const { item }: { item: ItemShort } = $props();
</script>

<div class="layout">
    <img class="icon" src={item.icon} alt="" />
    <span class="label">{item.name}</span>
    <span class="unit">{item.unit}</span>
</div>

<style>
.layout {
    display: grid;
    grid-template-columns: auto 1fr auto;
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
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
}

.unit {
    color: var(--clr-text-dim);
    margin-right: 0.25rem;
}
</style>

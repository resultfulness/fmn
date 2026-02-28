<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import Image from "$lib/components/atoms/image.svelte";

const { item }: { item: ItemShort } = $props();
</script>

<div class="layout">
    <Image src={item.icon} alt="" width={48} />
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

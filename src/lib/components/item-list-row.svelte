<script lang="ts">
import type { ItemShort } from "$lib/schemas/items";
import { Trash } from "@lucide/svelte";
import Button from "./button.svelte";

const {
    item,
    handleDelete,
}: { item: ItemShort; handleDelete: (id: number) => void } = $props();
</script>

<li class="item-row">
    <img class="item-row-icon" src={item.icon} alt={item.name + " icon"} />
    <span class="item-row-label">{item.name}</span>
    <Button onclick={() => handleDelete(item.item_id)} variant="alert" icon>
        <Trash />
    </Button>
</li>

<style>
.item-row {
    background-color: var(--clr-s1);
    padding: 0.5rem;
    display: flex;
    align-items: center;
    font-weight: 500;
    gap: 0.5rem;
}

.item-row-icon {
    display: block;
    max-width: 48px;
}

.item-row-label {
    flex: 1;
}
</style>

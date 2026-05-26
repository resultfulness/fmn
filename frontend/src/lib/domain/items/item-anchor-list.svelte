<script lang="ts">
import AnchorItem from "$lib/ui/molecules/anchor-item.svelte";
import ItemWide from "./item-wide.svelte";
import type { Item } from "./item";

const { items }: { items: Item[] } = $props();
</script>

<ul class="item-list">
    {#each items as item}
        <AnchorItem
            href={`/items/${item.item_id}/edit`}
            ariaLabel={`edit item ${item.name}`}
        >
            <ItemWide {item} />
        </AnchorItem>
    {/each}
</ul>

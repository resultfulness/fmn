<script lang="ts">
import IconTileGrid from "$lib/components/molecules/icon-tile-grid.svelte";
import type { ItemShort } from "$lib/schemas/items";

const { items }: { items: ItemShort[] | undefined } = $props();
</script>

{#if items === undefined}
    loading...
{:else if items.length <= 0}
    no items here
{:else}
    <IconTileGrid
        centerLabels
        tiles={items.map(({ icon, name }) => ({
            iconUrl: icon,
            label: name,
        }))}
    />
{/if}

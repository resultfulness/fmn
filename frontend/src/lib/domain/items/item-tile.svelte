<script lang="ts">
import IconTile from "$lib/ui/molecules/icon-tile.svelte";
import type { Item } from "./item";

interface ItemTileProps {
    item: Item;
    onclick: (id: number) => void;
}

const { item, onclick }: ItemTileProps = $props();
</script>

<IconTile
    centerLabel
    iconUrl={item.icon}
    label={item.name}
    onclick={() => onclick(item.item_id)}
/>

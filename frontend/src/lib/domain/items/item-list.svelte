<script lang="ts">
import type { Item } from "./item";
import ButtonItem from "$lib/ui/molecules/button-item.svelte";
import ItemWide from "./item-wide.svelte";

interface ItemListProps {
    items: Item[];
    onclick: (id: number) => void;
}

const { items, onclick }: ItemListProps = $props();
</script>

<ul class="item-list">
    {#each items as item}
        <ButtonItem onclick={() => onclick(item.item_id)}>
            <ItemWide {item} />
        </ButtonItem>
    {/each}
</ul>

<style>
ul {
    list-style-type: none;
    display: grid;
    gap: 0.5rem;
    padding: 0;
    margin: 0;
}
</style>

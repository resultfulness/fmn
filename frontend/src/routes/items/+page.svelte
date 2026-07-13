<script lang="ts">
import { onMount } from "svelte";
import type { Item } from "$lib/domain/items/item";
import { HeaderState } from "$lib/ui/header.svelte";
import ListPage from "$lib/ui/templates/list-page.svelte";
import FooterExtension from "$lib/ui/molecules/footer-extension.svelte";
import Search from "$lib/ui/molecules/search.svelte";
import IconButton from "$lib/ui/molecules/icon-button.svelte";
import ItemAnchorList from "$lib/domain/items/item-anchor-list.svelte";
import { Plus } from "@lucide/svelte";
import itemStore from "$lib/domain/items/store.svelte";

let searchterm = $state("");

const itemFound = (item: Item) =>
    item.name.toLowerCase().includes(searchterm.toLowerCase());

let itemsFiltered = $derived(itemStore.items.filter(itemFound));

onMount(() => {
    HeaderState.title = "items";
    delete HeaderState.backUrl;

    itemStore.load();
});
</script>

<ListPage>
    {#if itemsFiltered.length > 0}
        <ItemAnchorList items={itemsFiltered} />
    {:else}
        <div class="text-subtitle text-center">
            {#if searchterm}
                no items matching {searchterm}
            {:else}
                no items!
            {/if}
        </div>
    {/if}
</ListPage>
<FooterExtension>
    <Search bind:searchterm placeholder="search for items..." />
    <IconButton href="/items/new" icon={Plus} size={32} />
</FooterExtension>

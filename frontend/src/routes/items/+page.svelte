<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { type Item } from "$lib/schemas/items";
import { Plus } from "@lucide/svelte";
import Search from "$lib/components/molecules/search.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemLinkList from "$lib/components/organisms/item-link-list.svelte";
import { pushToast } from "$lib/components/toast.svelte";
import ListPage from "$lib/components/templates/list-page.svelte";

let items = $state<Item[]>();
let searchterm = $state("");

const itemsSearched = $derived(
    items?.filter(({ name }) => name.includes(searchterm))
);

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => pushToast(e, "error"));
});

HeaderState.title = "items";
delete HeaderState.backUrl;
</script>

<ListPage>
    {#if itemsSearched && itemsSearched.length > 0}
        <ItemLinkList items={itemsSearched} />
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

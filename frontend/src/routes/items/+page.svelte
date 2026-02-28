<script lang="ts">
import { onMount } from "svelte";
import api from "$lib/api";
import { type ItemShort } from "$lib/schemas/items";
import { Plus } from "@lucide/svelte";
import Search from "$lib/components/molecules/search.svelte";
import IconButton from "$lib/components/molecules/icon-button.svelte";
import FooterExtension from "$lib/components/molecules/footer-extension.svelte";
import { HeaderState } from "$lib/components/organisms/header.svelte";
import ItemLinkList from "$lib/components/organisms/item-link-list.svelte";

let items = $state<ItemShort[]>();
let searchterm = $state("");

const itemsSearched = $derived(
    items?.filter(({ name }) => name.includes(searchterm))
);

onMount(() => {
    api.items
        .readAll()
        .then(_items => (items = _items))
        .catch(e => alert(e));
});

HeaderState.title = "items";
delete HeaderState.backUrl;
</script>

<div class="items">
    <ItemLinkList items={itemsSearched} />
</div>
<FooterExtension>
    <Search bind:searchterm />
    <IconButton href="/items/new" icon={Plus} size={32} />
</FooterExtension>

<style>
.items {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
}
</style>

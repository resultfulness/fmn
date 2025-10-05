<script lang="ts">
import { onMount } from "svelte";
import data from "./data.svelte";
import actions from "./actions";

onMount(() => {
    actions.init();
    return () => actions.deinit();
});
</script>

<ul>
    {#each data.items as item}
        <li>
            {item.name}
            <button onclick={() => actions.deleteItem(item.item_id)}>
                delete
            </button>
        </li>
    {/each}
</ul>
<form onsubmit={actions.handleAddItem}>
    <input type="text" name="item-name" />
    <input type="text" name="item-icon" />
    <button>add</button>
</form>

<style>
ul {
    list-style-type: none;
    padding: 0;
}
</style>

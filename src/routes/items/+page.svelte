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
    {#each data.items.items as item}
        <li>
            {item.name}
            <button onclick={() => actions.deleteItem(item.item_id)}>
                delete test
            </button>
        </li>
    {/each}
</ul>
<button onclick={() => actions.addItem({ name: "milk2", icon: "" })}>
    add test
</button>

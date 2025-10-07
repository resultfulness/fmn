<script lang="ts">
import { onMount } from "svelte";
import actions from "./actions";
import data from "./data.svelte";

onMount(() => {
    actions.init();
    return () => actions.deinit();
});
</script>

carts!

{#each data.cartItems as item}
    <button onclick={() => actions.handleDeleteItem(item)}>
        {item.name}
    </button>
{/each}

{#each data.items as item}
    <button onclick={() => actions.handleAddItem(item)}>
        {item.name}
    </button>
{/each}

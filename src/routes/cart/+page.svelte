<script>
import { onMount } from "svelte";
import actions from "./actions";
import data from "./data.svelte";

onMount(() => {
    actions.init();
    return () => actions.deinit();
})
</script>

carts!

{#each data.cartItems as item}
    {item.name}
{/each}

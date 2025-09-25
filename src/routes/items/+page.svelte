<script lang="ts">
import { onMount } from "svelte";
import state from "./state.svelte";
import actions from "./actions";

onMount(() => {
    actions.init();
    return () => actions.deinit();
});
</script>

items!
{#each state.items as item}
    {item.name}
{/each}

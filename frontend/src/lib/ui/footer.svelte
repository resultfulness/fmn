<script lang="ts">
import { inSearchMode } from "$lib/search-mode.svelte";
import AppNavigation from "$lib/ui/app-navigation.svelte";
import { slide } from "svelte/transition";
</script>

{#if !inSearchMode()}
    <footer transition:slide={{ delay: 50 }}>
        <AppNavigation />
    </footer>
{/if}

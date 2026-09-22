<script module lang="ts">
type HeaderState = {
    title: string;
    backUrl?: string;
};

export const HeaderState = $state<HeaderState>({ title: "" });
</script>

<script lang="ts">
import { ArrowLeft } from "@lucide/svelte";
import IconButton from "$lib/ui/molecules/icon-button.svelte";
import { onMount } from "svelte";
import { inSearchMode } from "$lib/search-mode.svelte";
import { slide } from "svelte/transition";

const { title, backUrl } = $derived(HeaderState);

const { initialTitle }: { initialTitle: string } = $props();
onMount(() => (HeaderState.title = initialTitle));
</script>

{#if !inSearchMode()}
    <header transition:slide>
        <div>
            {#if backUrl}
                <IconButton
                    variant="secondary"
                    href={backUrl}
                    icon={ArrowLeft}
                    size={32}
                />
            {/if}
        </div>
        <h1 class="text-header text-center">{title}</h1>
        <div></div>
    </header>
{/if}

<style>
header {
    display: grid;
    grid-template-columns: 48px 1fr 48px;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem;
}

h1 {
    margin: 0;
}
</style>

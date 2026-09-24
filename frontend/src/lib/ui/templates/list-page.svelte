<script lang="ts">
import { getSavedScroll, setSavedScroll } from "$lib/saved-scroll.svelte";

const { children } = $props();

let page: HTMLDivElement = $state()!;

export function scrollToTop() {
    page.scrollTop = 0;
}

export function saveScrollState() {
    setSavedScroll(page.scrollTop);
}

export function restoreScrollState() {
    page.scrollBy({ behavior: "instant", top: getSavedScroll() });
}
</script>

<div class="page" bind:this={page}>{@render children()}</div>

<style>
.page {
    padding: 1rem;
    overflow-y: auto;
    flex: 1;
}

@media (prefers-reduced-motion: no-preference) {
    .page {
        scroll-behavior: smooth;
    }
}
</style>

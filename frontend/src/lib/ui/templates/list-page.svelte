<script lang="ts">
const { children } = $props();

let page: HTMLDivElement = $state()!;

export function scrollToTop() {
    page.scrollTop = 0;
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

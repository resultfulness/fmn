<script lang="ts">
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();
</script>

<div class="footer-extension">
    {@render children()}
</div>

<style>
.footer-extension {
    padding: 0.75rem;
    background-color: var(--clr-overlay);
    border-radius: var(--rounding) var(--rounding) 0 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}
</style>

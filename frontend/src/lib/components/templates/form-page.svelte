<script lang="ts">
import Image from "$lib/components/atoms/image.svelte";

let { children, icon, title } = $props();
</script>

<div class="page">
    <Image src={icon} alt="" width={128} />
    <h2 class="text-title line-clamp-2 text-center">
        {#if title.length > 0}
            {title}
        {:else}
            &nbsp;
        {/if}
    </h2>
    {@render children()}
</div>

<style>
.page {
    padding: 1rem 1rem 0;
    overflow-y: auto;

    > :global(* + *) {
        margin-top: 1rem;
    }
}

h2 {
    margin: 0;
}
</style>

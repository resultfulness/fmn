<script lang="ts">
import Image from "$lib/components/atoms/image.svelte";

let { children, icon, title } = $props();
</script>

<div class="page">
    <Image src={icon} alt="" width={128} />
    <h2 class="text-title line-clamp-2">
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
    padding: 1rem;
    overflow-y: auto;

    > :global(* + *) {
        margin-top: 1rem;
    }
}

h2 {
    text-align: center;
    margin: 0;
}
</style>

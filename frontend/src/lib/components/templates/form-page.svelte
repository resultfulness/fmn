<script lang="ts">
let { children, icon, title } = $props();
</script>

<div class="page">
    <img src={icon} alt="" />
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

img {
    width: 128px;
    aspect-ratio: 1;
    object-fit: cover;
    margin-inline: auto;
    display: block;
}
</style>

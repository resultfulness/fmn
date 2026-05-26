<script lang="ts">
import IconTile, { type IconTileProps } from "./icon-tile.svelte";

interface IconTileGridProps {
    tiles: IconTileProps[];
    centerLabels?: boolean;
}

const { tiles, centerLabels }: IconTileGridProps = $props();
</script>

<div class="icon-tile-grid">
    {#each tiles as props}
        <IconTile {...props} centerLabel={centerLabels} />
    {/each}
</div>

<style>
.icon-tile-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(104px, 100%), 1fr));
    gap: 0.5rem;
    margin-inline: auto;
}
</style>

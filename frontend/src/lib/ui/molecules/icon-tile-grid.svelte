<script lang="ts">
import IconTile, { type IconTileProps } from "./icon-tile.svelte";

interface IconTileGridProps {
    tiles: IconTileProps[];
    centerLabels?: boolean;
}

const { tiles, centerLabels }: IconTileGridProps = $props();
</script>

<div class="tile-grid">
    {#each tiles as props}
        <IconTile {...props} centerLabel={centerLabels} />
    {/each}
</div>

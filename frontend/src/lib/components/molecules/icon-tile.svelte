<script lang="ts">
export interface IconTileProps {
    iconUrl: string;
    label?: string;
    subtitle?: string;
    centerLabel?: boolean;
}

const { iconUrl, label, subtitle, centerLabel }: IconTileProps = $props();
</script>

<div class="icon-tile">
    <div class="image" style:background-image={`url(${iconUrl})`}></div>
    {#if label}
        <span class="text-label label" class:center-label={centerLabel}>
            {label}
        </span>
        {#if !centerLabel}
            <span class="text-subtitle subtitle">
                {#if subtitle}{subtitle}{:else}&nbsp;{/if}
            </span>
        {/if}
    {/if}
</div>

<style>
.icon-tile {
    border-radius: var(--rounding);
    background-color: var(--clr-surface);
    padding: 0.5rem;
    display: grid;
    grid-template-rows: 1fr auto auto;
    grid-template-columns: 1fr;
    gap: 0.25rem;
    justify-items: center;
    text-align: center;
    aspect-ratio: 1;
}

.label {
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
}

.center-label {
    overflow-wrap: break-word;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    height: 2lh;
    align-content: center;
}

.subtitle {
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
}

.image {
    width: fit-content;
    aspect-ratio: 1;
    height: 100%;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    border-radius: var(--rounding);
    overflow: hidden;
}
</style>

<script lang="ts">
interface IconTileProps {
    iconUrl: string;
    label?: string;
    subtitle?: string;
}

const { iconUrl, label, subtitle }: IconTileProps = $props();
</script>

<div class="icon-tile">
    <div class="image" style:background-image={`url(${iconUrl})`}></div>
    {#if label}
        <span class="label">{label}</span>
        <span class="subtitle">
            {#if subtitle}{subtitle}{:else}&nbsp;{/if}
        </span>
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

.label {
    font: var(--font-label);
}

.subtitle {
    font: var(--font-subtitle);
    color: var(--clr-text-dim);
}
</style>

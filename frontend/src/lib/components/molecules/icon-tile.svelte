<script lang="ts">
interface IconTileProps {
    iconUrl: string;
    label?: string;
    subtitle?: string;
}

const { iconUrl, label, subtitle }: IconTileProps = $props();
</script>

<div class="icon-tile">
    <img src={iconUrl} alt="" style:max-width={label ? "64px" : "100px"} />
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
    gap: 0.25rem;
    justify-content: center;
    text-align: center;
}

img {
    aspect-ratio: 1;
    object-fit: cover;
}

.label {
    font: var(--font-label);
}

.subtitle {
    font: var(--font-subtitle);
    color: var(--clr-text-dim);
}
</style>

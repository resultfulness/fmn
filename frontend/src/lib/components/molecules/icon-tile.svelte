<script lang="ts">
import type { MouseEventHandler } from "svelte/elements";

export interface IconTileProps {
    iconUrl: string;
    label?: string;
    subtitle?: string;
    centerLabel?: boolean;
    onclick: MouseEventHandler<HTMLButtonElement> | null;
}

const { iconUrl, label, subtitle, centerLabel, onclick }: IconTileProps =
    $props();
</script>

<button class="icon-tile" {onclick}>
    <div class="image" style:background-image={`url(${iconUrl})`}></div>
    {#if label}
        <div class="wrapper">
        <span
            class={[
                "text-label",
                "label",
                centerLabel && "center-label",
                subtitle ? "line-clamp-1" : "line-clamp-2",
            ]}
        >
            {label}
        </span>
        {#if !centerLabel && subtitle}
            <span class="text-subtitle subtitle">{subtitle}</span>
        {/if}
        </div>
    {/if}
</button>

<style>
.icon-tile {
    border-radius: var(--rounding);
    background-color: var(--clr-surface);
    box-shadow: var(--shadow);
    padding: 0.5rem;
    display: grid;
    grid-template-rows: 1fr auto auto;
    grid-template-columns: 1fr;
    gap: 0.25rem;
    justify-items: center;
    text-align: center;
    aspect-ratio: 1;
    border: none;
}

.wrapper {
    display: grid;
    place-items: center;
    height: 2lh;
    gap: 0.25rem;
}

.label {
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
    align-content: center;
}

.center-label {
    overflow-wrap: break-word;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    height: 2lh;
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

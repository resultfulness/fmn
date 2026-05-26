<script lang="ts">
interface ImageProps {
    src: string;
    alt?: string;
    width?: number;
}

const { src, alt, width }: ImageProps = $props();
</script>

<img {src} {alt} {width} />

<style>
img {
    aspect-ratio: 1;
    object-fit: cover;
    border-radius: var(--rounding);
    margin-inline: auto;
    display: block;
}
</style>

<script lang="ts">
interface IconProps {
    name: string;
    size?: number;
    color?: string;
    filled?: boolean;
}

let {
    name,
    size = 24,
    color = "inherit",
    filled = false,
}: IconProps = $props();
</script>

<span
    class="material-symbols-rounded"
    style:color
    style:font-size={size + "px"}
    style:font-variation-settings={`'FILL' ${filled ? 1 : 0}`}
>
    {name}
</span>

<style>
.material-symbols-rounded {
    display: grid;
    place-items: center;
    width: fit-content;
    font-variation-settings: 'FILL' 0;
}
</style>

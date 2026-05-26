<script lang="ts">
import Select, { type SelectProps } from "$lib/ui/elements/select.svelte";

interface SelectFieldProps extends SelectProps {
    label: string;
}

let { label, value = $bindable(), ...props }: SelectFieldProps = $props();
const id = $props.id();
</script>

<div class="select-field">
    <label for={id} class="text-label">{label}</label>
    <Select {...props} {id} bind:value />
</div>

<style>
.select-field {
    display: grid;
    gap: 0.5rem;
}
</style>

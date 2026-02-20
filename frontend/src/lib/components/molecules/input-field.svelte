<script lang="ts">
import Input, { type InputProps } from "$lib/components/atoms/input.svelte";

interface InputFieldProps extends InputProps {
    label: string;
}

let { value = $bindable(), label, ...props }: InputFieldProps = $props();
const id = $props.id();
</script>

<div class="input-field">
    <label for={id} class="text-label">{label}</label>
    <Input {...props} bind:value {id} />
</div>

<style>
.input-field {
    display: grid;
    gap: 0.5rem;
}
</style>

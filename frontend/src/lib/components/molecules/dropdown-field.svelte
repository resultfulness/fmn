<script lang="ts">
import Dropdown, {
    type DropdownProps,
} from "$lib/components/atoms/dropdown.svelte";

interface DropdownFieldProps extends DropdownProps {
    label: string;
}

let { label, value = $bindable(), ...props }: DropdownFieldProps = $props();
const id = $props.id();
</script>

<div class="dropdown-field">
    <label for={id} class="text-label">{label}</label>
    <Dropdown {...props} {id} bind:value />
</div>

<style>
.dropdown-field {
    display: grid;
    gap: 0.5rem;
}
</style>

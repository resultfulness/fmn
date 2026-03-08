<script lang="ts">
import Textarea, {
    type TextareaProps,
} from "$lib/components/atoms/textarea.svelte";

interface TextareaFieldProps extends TextareaProps {
    label: string;
}

let { value = $bindable(), label, ...props }: TextareaFieldProps = $props();
const id = $props.id();
</script>

<div class="textarea-field">
    <label for={id} class="text-label">{label}</label>
    <Textarea {...props} bind:value {id} />
</div>

<style>
.textarea-field {
    display: grid;
    gap: 0.5rem;
}
</style>

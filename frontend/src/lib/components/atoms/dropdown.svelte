<script lang="ts">
export interface DropdownProps {
    options: string[];
    labels?: string[];
    placeholder: string;
    id?: string;
    required?: boolean;
    name?: string;
    value?: string;
}

let {
    id,
    options,
    labels = [],
    placeholder,
    required,
    name,
    value = $bindable(),
}: DropdownProps = $props();
</script>

<select {id} {required} {name} bind:value>
    <option value="" disabled selected hidden>{placeholder}</option>
    {#each options as option, i}
        <option value={option}>{labels[i] ?? option}</option>
    {/each}
</select>

<style>
select {
    background-color: var(--clr-surface);
    border: none;
    border-radius: var(--rounding);
    padding: 0.5rem 1rem;
    box-shadow: var(--shadow);
    font: var(--font-content);
    color: var(--clr-text);
}

select:focus {
    outline: 2px solid var(--clr-primary);
}

select:invalid {
    color: var(--clr-text-dim);
}
</style>

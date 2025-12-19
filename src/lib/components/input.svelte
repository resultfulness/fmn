<script lang="ts">
import { X, type Icon as IconType } from "@lucide/svelte";

interface InputProps {
    value: string;
    type?: string;
    id?: string;
    label?: string;
    name?: string;
    placeholder?: string;
    required?: boolean;
    error?: string;
    ariaLabel?: string;
    clearable?: boolean;
    icon?: typeof IconType;
}

const uid = $props.id();

let {
    value = $bindable(),
    type = "text",
    id = uid,
    label,
    name,
    placeholder,
    required,
    error,
    ariaLabel,
    clearable,
    icon,
}: InputProps = $props();
</script>

<div class="input-group">
    {#if label}
        <label for={id}>
            <span>{label}</span>
        </label>
    {/if}
    <div class="input-field">
        {#if icon}
            <div class="input-icon">
                <svelte:boundary>
                    {@const Icon = icon}
                    <Icon />
                </svelte:boundary>
            </div>
        {/if}
        <input
            {type}
            {name}
            {placeholder}
            {id}
            {required}
            aria-label={ariaLabel}
            bind:value
            class="input"
            class:input--icon={icon !== undefined}
            class:input--error={error}
        />
        {#if clearable && value.length > 0}
            <button
                class="input-clear"
                onclick={() => (value = "")}
                type="button"
            >
                <X color="var(--clr-text)" strokeWidth={1.5} />
            </button>
        {/if}
    </div>
    {#if error}
        <span class="input-group-error">{error}</span>
    {/if}
</div>

<style>
.input-group {
    display: grid;
}

.input-field {
    position: relative;
    display: grid;
}

.input {
    padding: 0.5rem 0.5rem;
    border-radius: 0.5rem;
    border: 0.1rem solid var(--clr-outline);
    background-color: var(--clr-s1);
    color: inherit;
    font: inherit;
}

.input:focus {
    outline: 0;
    border-color: var(--clr-primary);
}

.input::placeholder {
    color: var(--clr-outline);
}

.input--error {
    border-color: var(--clr-error) !important;
}

.input--icon {
    padding-left: 2.5rem;
}

.input-group-error {
    color: var(--color-error);
}

.input-icon {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: transparent;
    border: 0;
    display: grid;
    place-items: center;
    padding-inline: 0.5rem;
}

.input-clear {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    background: transparent;
    border: 0;
    display: grid;
    place-items: center;
    padding-inline: 0.5rem;
    cursor: pointer;
}
</style>

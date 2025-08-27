<script lang="ts">
import Icon from "./icon.svelte";

interface InputProps {
    type?: string;
    placeholder?: string;
    label?: string;
    id: string;
    value: string;
    required?: boolean;
    error?: string;
    ariaLabel?: string;
    showClear?: boolean;
    action?: {
        iconName: string;
        title: string;
        on: () => void;
    };
}

let {
    type = "text",
    placeholder,
    label,
    id,
    required,
    value = $bindable(),
    error,
    ariaLabel,
    showClear = false,
    action,
}: InputProps = $props();
</script>

<div class="input-group">
    {#if label}
        <label for={id}>
            <span>{label}</span>
        </label>
    {/if}
    <div class="input-field">
        <input
            {type}
            {placeholder}
            {id}
            {required}
            aria-label={ariaLabel}
            bind:value
            class="input"
            class:input--error={error}
        />
        {#if showClear && value.length > 0}
            <button
                class="input__clear"
                onclick={() => (value = "")}
                type="button"
            >
                <Icon name="close" />
            </button>
        {/if}
        {#if action}
            <button
                class="input__action"
                onclick={() => action.on()}
                style:right={showClear && value.length > 0 ? "2rem" : 0}
                title={action.title}
                type="button"
            >
                <Icon name={action.iconName} />
            </button>
        {/if}
    </div>
    {#if error}
        <span class="input-group__error">{error}</span>
    {/if}
</div>

<style>
.input-group {
    display: grid;
    gap: 0.5rem;
}

.input-field {
    position: relative;
    display: grid;
}

.input {
    padding: 0.75rem;
    border-radius: 0.5rem;
    border: 2px solid var(--color-outline);
    background-color: var(--color-surface0);
}

.input:focus {
    outline: 0;
    border: 2px solid var(--color-primary);
}

.input::placeholder {
    color: var(--color-outline);
}

.input--error {
    border: 2px solid var(--color-error) !important;
}

.input-group__error {
    color: var(--color-error);
}

.input__clear {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    padding-inline: 1rem;
    background: transparent;
    border: 0;
}

.input__action {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    padding-inline: 1rem;
    background: transparent;
    border: 0;
}
</style>

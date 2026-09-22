<script lang="ts">
import { X, type Icon as IconType } from "@lucide/svelte";
import type { HTMLInputTypeAttribute } from "svelte/elements";

export interface InputProps {
    value?: string | number;
    type?: HTMLInputTypeAttribute;
    id?: string;
    name?: string;
    placeholder?: string;
    required?: boolean;
    ariaLabel?: string;
    clearable?: boolean;
    onclear?: () => void;
    icon?: typeof IconType;
    endText?: string;
    onfocus?: () => void;
    onblur?: () => void;
    disabled?: boolean;
}

let {
    value = $bindable(),
    type = "text",
    id,
    name,
    placeholder,
    required,
    ariaLabel,
    clearable,
    onclear,
    icon,
    endText,
    onfocus,
    onblur,
    disabled,
}: InputProps = $props();
</script>

<div class="input-wrapper">
    {#if icon}
        <div class="icon">
            <svelte:boundary>
                {@const Icon = icon}
                <Icon color="var(--clr-text-dim)" />
            </svelte:boundary>
        </div>
    {/if}
    <input
        {disabled}
        {type}
        {name}
        {placeholder}
        {id}
        {required}
        aria-label={ariaLabel}
        bind:value
        class="input text-content"
        {onfocus}
        {onblur}
    />
    {#if clearable && typeof value === "string"}
        <button
            class="clear"
            onclick={e => {
                value = "";
                onclear?.();
                e.currentTarget.blur();
            }}
            type="button"
        >
            <X color="var(--clr-text)" strokeWidth={1.5} />
        </button>
    {/if}
    {#if endText}
        <span class="end-text text-subtitle">{endText}</span>
    {/if}
</div>

<style>
.input-wrapper {
    position: relative;
    display: grid;
    box-shadow: var(--shadow);
    border-radius: var(--rounding);
    overflow: hidden;
    flex: 1;
}

.input {
    padding: 0.5rem 1rem;
    background-color: var(--clr-surface);
    color: var(--clr-text);
    border: none;
}

.input-wrapper:has(.input:disabled) {
    filter: grayscale(0.25);
    opacity: 0.5;
}

.input:focus-visible {
    outline: none;
}

.input-wrapper:focus-within {
    outline: 2px solid var(--clr-primary);
}

.input::placeholder {
    color: var(--clr-text-dim);
}

.input-wrapper:has(.icon) .input {
    padding-left: 2.5rem;
}

.input-wrapper:has(.clear) .input {
    padding-right: 2.5rem;
}

.input-wrapper .icon {
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

.input-wrapper .clear {
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

.input-wrapper .end-text {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    text-align: right;
    display: grid;
    place-items: center;
    padding-inline: 0.5rem;
}

.input[type="number"]::-webkit-outer-spin-button,
.input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.input[type="number"] {
    -moz-appearance: textfield;
}
</style>

<script lang="ts">
export interface TextareaProps {
    value?: string;
    id?: string;
    name?: string;
    placeholder?: string;
    rows?: number;
    readonly?: boolean;
}

let {
    value = $bindable(),
    id,
    name,
    placeholder,
    rows = 5,
    readonly,
}: TextareaProps = $props();
</script>

<div class="textarea-wrapper" class:readonly>
    {#if !readonly}
        <div class="resize-handle">
            <div class="handle-part-1"></div>
            <div class="handle-part-2"></div>
        </div>
    {/if}
    <textarea
        class="textarea text-content"
        bind:value
        {id}
        {name}
        {placeholder}
        {rows}
        {readonly}
        tabindex={readonly ? -1 : 0}
    ></textarea>
</div>

<style>
.textarea-wrapper {
    position: relative;
    display: grid;
    box-shadow: var(--shadow);
    border-radius: var(--rounding);
    overflow: hidden;

    &:focus-within {
        outline: 2px solid var(--clr-primary);
    }

    &.readonly:focus-within {
        outline: none;
    }
}

.textarea {
    background-color: var(--clr-surface);
    color: var(--clr-text);
    padding: 0.5rem 1rem;
    border: none;
    resize: vertical;
    max-height: 10lh;
    min-height: 1lh;

    &::placeholder {
        color: var(--clr-text-dim);
    }

    &:focus-visible {
        outline: none;
    }
}

.resize-handle {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 32px;
    height: 32px;
    pointer-events: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    transform: rotate(-45deg);
}

.handle-part-1,
.handle-part-2 {
    background-color: var(--clr-text-dim);
    border-radius: 100vw;
}

.handle-part-1 {
    width: 16px;
    height: 2px;
}

.handle-part-2 {
    width: 8px;
    height: 2px;
}
</style>

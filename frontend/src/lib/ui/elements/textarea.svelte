<script lang="ts">
export interface TextareaProps {
    value?: string;
    id?: string;
    name?: string;
    placeholder?: string;
    rows?: number;
    readonly?: boolean;
    growing?: boolean;
}

let {
    value = $bindable(),
    id,
    name,
    placeholder,
    rows = 5,
    readonly,
    growing,
}: TextareaProps = $props();
</script>

<div class="textarea-wrapper" class:readonly class:growing data-value={value}>
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

    .textarea {
        border: none;
        background-color: var(--clr-surface);
        color: var(--clr-text);
        padding: 0.5rem 1rem;

        &::placeholder {
            color: var(--clr-text-dim);
        }

        &:focus-visible {
            outline: none;
        }
    }
}

.textarea-wrapper.growing {
    &::after {
        content: attr(data-value) " ";
        white-space: pre-wrap;
        visibility: hidden;

        border: none;
        background-color: var(--clr-surface);
        color: var(--clr-text);
        padding: 0.5rem 1rem;
    }

    .textarea {
        resize: none;
        overflow: hidden;
    }

    &::after,
    .textarea {
        grid-area: 1 / 1 / 2 / 2;
    }
}
</style>

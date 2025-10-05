<script lang="ts">
import Icon from "./icon.svelte";
import { toastStore } from "$lib/stores/toast-store.svelte";
</script>

<ol class="toast-list">
    {#each toastStore.toasts as toast (toast.id)}
        <div
            class="toast"
            class:toast--yayy={toast.type === "yayy"}
            class:toast--warn={toast.type === "warn"}
            class:toast--uhoh={toast.type === "uhoh"}
            class:toast--info={toast.type === "info"}
        >
            <button
                onclick={() => toastStore.remove(toast.id)}
                aria-label="close toast"
                class="toast__close"
            ></button>
            <div class="toast__icon">
                <Icon
                    name={toast.type === "yayy"
                        ? "check"
                        : toast.type === "warn"
                          ? "warning"
                          : toast.type === "uhoh"
                            ? "error"
                            : "info"}
                    size={40}
                />
            </div>
            <p class="toast__message">{toast.message}</p>
            <div
                class="toast__timeout"
                style:animation-duration={`${toast.duration}ms`}
            ></div>
        </div>
    {/each}
</ol>

<style>
.toast-list {
    z-index: 9;
    position: absolute;
    left: 0;
    right: 0;
    top: 5rem;
    margin: 0 auto;
    list-style-type: none;
    pointer-events: none;
    padding: 1rem;
}

.toast {
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    color: var(--clr-prim-text);
    border-radius: 1rem;
    box-shadow: 0 1px 4px 0 rgb(0 0 0 / 0.1);
}

.toast__timeout {
    position: absolute;
    inset: 0;
    background-color: black;
    opacity: 0.25;
    width: 0%;
    pointer-events: none;
    animation: fillTimeout 0s linear;
}

@keyframes fillTimeout {
from {
    width: 0%;
} to {
    width: 100%;
}
}

.toast + .toast {
    margin-top: 1rem;
}

.toast--yayy {
    background-color: var(--clr-yayy);
}
.toast--warn {
    background-color: var(--clr-warn);
}
.toast--uhoh {
    background-color: var(--clr-uhoh);
}
.toast--info {
    background-color: var(--clr-dark);
    color: var(--clr-text);
}

.toast__icon {
    padding: 1rem;
}

.toast__message {
    overflow: hidden;
    font-size: 1.1rem;
}

.toast__close {
    position: absolute;
    inset: 0;
    border: 0;
    background: 0;
    pointer-events: all;
}
</style>

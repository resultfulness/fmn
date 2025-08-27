<script lang="ts">
import type { ApiError } from "$lib/api";
import api from "$lib/api";
import Button from "$lib/components/button.svelte";
import { showConfirm } from "$lib/components/confirm.svelte";
import Drawer from "$lib/components/drawer.svelte";
import Icon from "$lib/components/icon.svelte";
import Input from "$lib/components/input.svelte";
import { showToast } from "$lib/toast.svelte";
import type { Item } from "$lib/types";
import { getContext, onMount } from "svelte";
import unfocusOnMobileKeyboardHidden from "$lib/mobile-unfocus";
import type { LayoutHeader } from "../+layout.svelte";
import app from "$lib/app.svelte";
import Search from "$lib/components/search.svelte";
import autofill_url from "$lib/autofill-url";
import {
    PUBLIC_ICON_URL_LEAD,
    PUBLIC_ICON_URL_TRAIL,
} from "$env/static/public";

async function fetchItems() {
    app.state.isLoading++;
    await app.updateItems();
    app.state.isLoading--;
}

let searchterm = $state("");
let filteredItems = $derived(
    app.state.items.items.filter(item => item.name.includes(searchterm))
);

let form = $state({
    name: "",
    icon: "",
    error: {
        name: "",
        icon: "",
    },
});
let itemEdited: Item | undefined = $state();
let submitDisabled = $derived(
    (form.name === itemEdited?.name || form.name.length < 1) &&
        (form.icon === itemEdited?.icon || form.icon.length < 1)
);

// {{{ edit
let editDrawer: HTMLDialogElement = $state()!;
function edit(item: Item) {
    itemEdited = item;
    form.name = itemEdited.name;
    form.icon = itemEdited.icon;
    editDrawer.show();
}
function noedit() {
    editDrawer.close();
    form.name = "";
    form.icon = "";
    form.error = { name: "", icon: "" };
    itemEdited = undefined;
}
async function handleEdit(e: SubmitEvent) {
    e.preventDefault();
    if (!itemEdited) return;
    form.error = { name: "", icon: "" };

    if (!form.name) {
        form.error.name = "name required";
        return;
    }

    if (!form.icon) {
        form.error.icon = "icon required";
        return;
    }

    if (submitDisabled) {
        return;
    }

    try {
        const index = app.state.items.items.findIndex(
            item => item.item_id === itemEdited!.item_id
        );
        if (index >= 0) {
            app.state.items.items[index] = {
                item_id: itemEdited.item_id,
                name: form.name,
                icon: form.icon,
            };
        }
        const data = await api.items.patch({
            item_id: itemEdited.item_id,
            name: form.name,
            icon: form.icon,
        });
        if (data) {
            noedit();
            showToast("item saved", "success");
        }
    } catch (e) {
        const ae = e as ApiError;
        switch (ae.status) {
            case 409:
                form.error.name = ae.message;
                break;
        }
    }
}
// }}}

// {{{ add
let addDrawer: HTMLDialogElement = $state()!;
function add() {
    form.name = searchterm;
    addDrawer.show();
}
function noadd() {
    addDrawer.close();
    form.name = "";
    form.icon = "";
    form.error = { name: "", icon: "" };
}
async function handleAdd(e: SubmitEvent) {
    e.preventDefault();
    form.error = { name: "", icon: "" };

    if (!form.name) {
        form.error.name = "name required";
        return;
    }

    if (!form.icon) {
        form.error.icon = "icon required";
        return;
    }

    try {
        const data = await api.items.new({
            name: form.name,
            icon: form.icon,
        });
        if (data) {
            app.state.items.items.push(data);
            noadd();
            showToast("item added", "success");
        }
    } catch (e) {
        const ae = e as ApiError;
        switch (ae.status) {
            case 409:
                form.error.name = ae.message;
                break;
        }
    }
}
/// }}}

// {{{ delete
async function handleDelete(id: number) {
    if (itemEdited === undefined) return;

    if (
        await showConfirm(
            "delete confirmation",
            `are you sure you want to remove '${itemEdited.name}'?`
        )
    ) {
        try {
            app.state.items.items = app.state.items.items.filter(
                item => item.item_id !== id
            );
            const res = await api.items.delete(id);
            if (res) {
                noedit();
                showToast(`item '${res.name}' deleted`, "success");
            }
        } catch (e) {}
    }
}
// }}}

// {{{ search
let searchMode = $state(false);
let searchDrawer: HTMLDialogElement = $state()!;
function search() {
    searchMode = true;
    searchDrawer.show();
}
function nosearch() {
    searchMode = false;
    searchDrawer.close();
}
// }}}

onMount(() => {
    fetchItems();

    const header = getContext<LayoutHeader>("header");
    header.title = "items";

    unfocusOnMobileKeyboardHidden("items-search");
});

function autofill_url_staticicon() {
    form.icon = autofill_url(
        PUBLIC_ICON_URL_LEAD,
        form.icon,
        PUBLIC_ICON_URL_TRAIL
    );
}
</script>

{#if app.state.items}
    <ul class="card-list">
        {#if filteredItems.length > 0}
            {#each filteredItems as item}
                <li>
                    <button class="card item" onclick={() => edit(item)}>
                        <img
                            src={item.icon}
                            alt={item.name + " icon"}
                            class="item__icon"
                        />
                        <p class="card__title">{item.name}</p>
                        <div class="item__edit-icon">
                            <Icon name="edit" size={28} />
                        </div>
                    </button>
                </li>
            {/each}
        {:else}
            <p class="empty-label">no items for '{searchterm}'</p>
        {/if}
    </ul>
{/if}
<Search
    id="items-search"
    bind:searchterm
    placeholder="search for items..."
    onadd={add}
/>
<Drawer bind:ref={editDrawer} onclose={noedit}>
    <form class="form" onsubmit={handleEdit}>
        <header class="form__header">
            <h2 class="form__title">editing item</h2>
            <Button type="button" onclick={noedit} style="icon">
                <Icon name="close" size={32} />
            </Button>
        </header>
        <Input id="item-name" bind:value={form.name} error={form.error.name} />
        <Input id="item-icon" bind:value={form.icon} error={form.error.icon} />
        <div class="form__submit">
            <Button
                fillwidth
                style="alert"
                type="button"
                onclick={() => handleDelete(itemEdited!.item_id)}
            >
                delete
            </Button>
            <Button
                fillwidth
                disabled={submitDisabled}
                tooltip={submitDisabled ? "change something first!" : undefined}
            >
                save
            </Button>
        </div>
    </form>
</Drawer>
<Drawer bind:ref={addDrawer} onclose={noadd}>
    <form class="form" onsubmit={handleAdd}>
        <header class="form__header">
            <h2 class="form__title">adding item</h2>
            <Button type="button" onclick={noadd} style="icon">
                <Icon name="close" size={32} />
            </Button>
        </header>
        <Input
            id="item-name"
            label="name:"
            bind:value={form.name}
            error={form.error.name}
        />
        <Input
            id="item-icon"
            label="icon url:"
            bind:value={form.icon}
            error={form.error.icon}
            action={{
                iconName: "password_2",
                title: "autofill icon url",
                on: autofill_url_staticicon,
            }}
        />
        <div class="form__submit">
            <Button fillwidth>add</Button>
        </div>
    </form>
</Drawer>

<style>
.item {
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    justify-content: start;
    align-items: center;
    gap: 1rem;
}

.item__icon {
    width: 32px;
    object-fit: cover;
}

.item__edit-icon {
    margin-left: auto;
    margin-right: 0.25rem;
    display: grid;
}
</style>

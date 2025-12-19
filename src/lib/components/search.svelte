<script lang="ts">
import Input from "./input.svelte";
import { Search } from "@lucide/svelte";

interface SearchProps {
    searchterm: string;
    padding?: string;
}

let { searchterm = $bindable(), padding = "0" }: SearchProps = $props();
</script>

<div class="search" style:padding>
    <Input
        icon={Search}
        clearable
        placeholder="search for items..."
        bind:value={searchterm}
    />
</div>

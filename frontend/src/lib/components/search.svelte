<script lang="ts">
import Input from "$lib/components/atoms/input.svelte";
import { Search } from "@lucide/svelte";

interface SearchProps {
    searchterm: string;
}

let { searchterm = $bindable() }: SearchProps = $props();
</script>

<Input
    icon={Search}
    clearable
    placeholder="search for items..."
    bind:value={searchterm}
/>

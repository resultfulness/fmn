<script lang="ts">
import Input from "$lib/components/atoms/input.svelte";
import { Search } from "@lucide/svelte";

interface SearchProps {
    searchterm: string;
    placeholder: string;
}

let { searchterm = $bindable(), placeholder }: SearchProps = $props();
</script>

<Input
    icon={Search}
    clearable
    {placeholder}
    bind:value={searchterm}
/>

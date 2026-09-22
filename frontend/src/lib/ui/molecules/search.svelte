<script lang="ts">
import Input, { type InputProps } from "$lib/ui/elements/input.svelte";
import { Search } from "@lucide/svelte";

interface SearchProps extends InputProps {
    searchterm: string;
}

let { searchterm = $bindable(), ...props }: SearchProps = $props();
</script>

<Input icon={Search} clearable bind:value={searchterm} {...props} />

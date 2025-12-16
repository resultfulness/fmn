<script lang="ts">
import { ChefHat, Salad, ShoppingCart } from "@lucide/svelte";
import NavbarLink from "./navbar-link.svelte";
</script>

<footer>
    <nav>
        <NavbarLink href="/" icon={ShoppingCart} label="cart" />
        <NavbarLink href="/recipes" icon={ChefHat} label="recipes" />
        <NavbarLink href="/items" icon={Salad} label="items" />
    </nav>
</footer>

<style>
footer {
    background-color: var(--clr-s1);
}

nav {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    justify-items: stretch;
    align-content: center;
}
</style>

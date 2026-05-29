let lastRecipe = $state(-1);

export function getLastRecipe() {
    return lastRecipe;
}

export function setLastRecipe(id: number) {
    lastRecipe = id;
}

let searchMode = $state(false);

export function inSearchMode() {
    return searchMode;
}

export function setSearchMode(on: boolean) {
    searchMode = on;
}

let savedScroll = $state(0);

export function setSavedScroll(scroll :number) {
    savedScroll = scroll;
}

export function getSavedScroll() {
    return savedScroll;
}

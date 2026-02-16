function updateStorageStringify(key: string, value: any) {
    localStorage.setItem(key, JSON.stringify(value));
}

export default function createLocalStorage<T>(key: string, initValue: T) {
    let init = initValue;

    const localStorageValue = localStorage.getItem(key);
    if (localStorageValue === null) {
        updateStorageStringify(key, initValue);
    } else {
        init = JSON.parse(localStorageValue);
    }

    let value = $state(init);
    $effect(() => updateStorageStringify(key, value));
}

import { get, writable, type Updater } from "svelte/store";

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

    const store = writable(init);

    return {
        subscribe: store.subscribe,
        set: (value: T) => {
            store.set(value);
            updateStorageStringify(key, value);
        },
        update: (updater: Updater<T>) => {
            store.update(updater)
            updateStorageStringify(key, get(store));
        }
    }
}

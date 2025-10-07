import { get, writable, type Updater } from "svelte/store";

type AsyncUpdater<T> = (value: T) => Promise<T>;

export function optimisticStore<T>(initial?: T) {
    const store = writable(initial);
    const { subscribe, set, update } = store;
    let prev: T;

    return {
        subscribe,
        set,
        update,
        async updateOptimistic(
            optimisticUpdater: Updater<T>,
            updater: AsyncUpdater<T>,
        ) {
            prev = structuredClone(get(store));

            this.update(optimisticUpdater);

            try {
                const final = await updater(prev);
                this.set(final);
            } catch (e: any) {
                this.undo();
                throw e;
            }
        },
        undo() {
            if (prev) {
                store.set(prev);
            }
        }
    }
}

export function proxify<T>(val: T) {
    let p = $state(val);
    return p;
}

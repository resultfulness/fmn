export function proxify<T>(val: T) {
    let _ = $state(val);
    return _;
}

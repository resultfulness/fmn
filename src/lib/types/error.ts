export const Errors = Object.freeze({
    ApiAccessError: new Error("error accessing api"),
    Items: {
        NotFoundError: new Error("item not found"),
        AlreadyExistsError: new Error("item already exists"),
    },
});

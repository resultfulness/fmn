export async function handleGenericErrors(response: any): Promise<any> {
    if (!response) {
        throw new Error("request failed");
    }
    if (response.ok) {
        return response;
    }
    throw new Error((await response.json()).error || "unknown error")
}

import { pushToast } from "$lib/components/toast.svelte";
import type { $ZodIssue } from "zod/v4/core";

function issueToMessage(issue: $ZodIssue) {
    let path = issue.path.at(0);
    if (path === undefined) {
        return issue.message;
    }
    return `${path.toString()}: ${issue.message}`;
}

export function printIssues(issues: $ZodIssue[]) {
    for (const issue of issues) {
        pushToast(issueToMessage(issue), "error");
    }
}

export default function autofill_url(lead: string, fill: string, trail: string): string {
    fill = fill.trim();

    let res = "";
    if (!fill.startsWith(lead)) {
        res += lead;
    }
    res += fill;
    if (!fill.endsWith(trail)) {
        res += trail;
    }
    return res;
}

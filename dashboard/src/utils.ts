export function isEmpty(value: any) {
    if (typeof value === 'string') {
        return value.trim() === '';
    } else if (Array.isArray(value)) {
        return value.length === 0;
    }
    return value === null || value === undefined || value === '';
}

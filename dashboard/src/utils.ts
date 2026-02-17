export function isEmpty(value: any) {
    if (typeof value === 'string') {
        return value.trim() === '';
    }
    return value === null || value === undefined || value === '';
}

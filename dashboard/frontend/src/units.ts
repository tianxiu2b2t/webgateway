const BYTE_UNITS = ['', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y'];
const NUMBER_UNITS = ['', 'K', 'M', 'B', 'T', 'P', 'E', 'Z', 'Y'];

export function formatBytes(bytes: number, decimals = 2): string {
    if (bytes === 0) return '0 iB';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))}${BYTE_UNITS[i]}iB`.trim();
}

export function formatNumber(number: number, decimals = 2): string {
    if (number === 0) return '0';

    const dm = decimals < 0 ? 0 : decimals;
    const i = Math.floor(Math.log(number) / Math.log(1000));
    return `${parseFloat((number / Math.pow(1000, i)).toFixed(dm))}${NUMBER_UNITS[i]}`.trim();
}

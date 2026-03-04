import { gotWithAuth } from '../constant';
import type { APIResponse } from '../types';
import type {
    CreateCertificateContent,
    CreateCertificateType,
} from '../types/certificate';

const prefix = 'certificates';

export async function create(
    type: CreateCertificateType,
    content: CreateCertificateContent,
    name?: string,
) {
    const resp = (await (
        await gotWithAuth.post(`${prefix}/create`, {
            json: {
                name,
                type,
                content,
            },
        })
    ).json()) as APIResponse<any>;
    return resp;
}

export async function total() {
    const resp = (await (
        await gotWithAuth.get(`${prefix}/total`)
    ).json()) as APIResponse<number>;
    return resp.data;
}

// export async function fetch(page: number, perPage: number) {
//     const resp = (await (
//         await gotWithAuth.get(`${prefix}/page`, {
//             searchParams: {
//                 page,
//                 size: perPage,
//             },
//         })
//     ).json()) as APIResponse<DatabaseDNSProvider[]>;
//     return resp.data;
// }

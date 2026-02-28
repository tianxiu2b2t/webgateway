import { gotWithAuth } from '../constant';
import type { APIResponse } from '../types';
import type { DatabaseDNSProvider } from '../types/dnsproviders';

export async function create(
    name: string,
    domains: string[],
    provider_type: string,
    provider_config: any,
) {
    const resp = (await (
        await gotWithAuth.post('dnsproviders/create', {
            json: {
                name,
                domains,
                type: provider_type,
                config: provider_config,
            },
        })
    ).json()) as APIResponse<any>;
    return resp;
}

export async function total() {
    const resp = (await (
        await gotWithAuth.get('dnsproviders/total')
    ).json()) as APIResponse<number>;
    return resp.data;
}

export async function fetch(page: number, perPage: number) {
    const resp = (await (
        await gotWithAuth.get('dnsproviders/page', {
            searchParams: {
                page,
                size: perPage,
            },
        })
    ).json()) as APIResponse<DatabaseDNSProvider[]>;
    return resp.data;
}

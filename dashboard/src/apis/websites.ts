import { gotWithAuth } from '../constant';
import type { APIResponse, Website, WebsiteCreateRequest } from '../types';

const prefix = 'websites';

export async function createWebsite(website: WebsiteCreateRequest) {
    const resp = (await (
        await gotWithAuth.post(`${prefix}/create`, {
            json: website,
        })
    ).json()) as APIResponse<Website>;
    return resp;
}

export async function getWebsites(): Promise<Website[]> {
    const resp = (await (
        await gotWithAuth.get(`${prefix}`)
    ).json()) as APIResponse<Website[]>;
    return resp.data;
}

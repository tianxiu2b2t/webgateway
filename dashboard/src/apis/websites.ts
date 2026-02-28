import { gotWithAuth } from '../constant';
import type { APIResponse, Website, WebsiteCreateRequest } from '../types';

export async function createWebsite(
    website: WebsiteCreateRequest,
): Promise<Website> {
    const resp = (await (
        await gotWithAuth.post('websites/create', {
            json: website,
        })
    ).json()) as APIResponse<Website>;
    return resp.data;
}

export async function getWebsites(): Promise<Website[]> {
    const resp = (await (
        await gotWithAuth.get('websites')
    ).json()) as APIResponse<Website[]>;
    return resp.data;
}

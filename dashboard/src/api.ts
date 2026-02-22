import { gotWithAuth } from './constant';
import type {
    APIResponse,
    Log,
    UserInfo,
    Website,
    WebsiteCreateRequest,
} from './types';

let userinfos: Record<string, UserInfo> = {};

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

export async function getLogTotals(): Promise<number> {
    const resp = (await (
        await gotWithAuth.get('logs/total')
    ).json()) as APIResponse<number>;
    return resp.data;
}

export async function fetchLog(limit: number, page: number): Promise<Log[]> {
    const resp = (await (
        await gotWithAuth.get('logs/page', {
            searchParams: {
                limit,
                page,
            },
        })
    ).json()) as APIResponse<Log[]>;
    return resp.data;
}

export async function getUserInfo(user_id: string): Promise<UserInfo> {
    if (userinfos[user_id]) {
        return userinfos[user_id];
    }
    const resp = (await (
        await gotWithAuth.get('auth/info', {
            searchParams: {
                user_id,
            },
        })
    ).json()) as APIResponse<UserInfo>;
    userinfos[user_id] = resp.data;
    return resp.data;
}

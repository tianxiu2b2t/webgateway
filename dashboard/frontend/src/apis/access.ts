import { gotWithAuth } from '../constant';
import type { APIResponse } from '../types';
import type { AccessInfo } from '../types/access';

const prefix = 'access';

export async function get_qps(interval: number = 5) {
    const resp = (await (
        await gotWithAuth.get(`${prefix}/qps`, {
            searchParams: {
                interval,
            },
        })
    ).json()) as APIResponse<any>;
    return resp;
}

export async function get_access_info(in_days: number = 1) {
    const resp = (await (
        await gotWithAuth.get(`${prefix}/info`, {
            searchParams: {
                in_days,
            },
        })
    ).json()) as APIResponse<AccessInfo>;
    return resp.data;
}

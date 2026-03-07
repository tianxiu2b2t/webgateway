import { gotWithAuth } from '../constant';
import type { APIResponse, UserInfo } from '../types';

let userinfos: Record<string, UserInfo> = {};
let pendingRequests: Record<string, Promise<UserInfo>> = {}; // 新增：存储正在进行的请求

export async function getUserInfo(user_id: string): Promise<UserInfo> {
    // 如果缓存中已有数据，直接返回
    if (userinfos[user_id]) {
        return userinfos[user_id];
    }

    // 如果已经有相同 user_id 的请求正在进行，返回该 Promise
    if (pendingRequests[user_id]) {
        return pendingRequests[user_id];
    }

    // 否则，发起新请求，并将 Promise 存入 pendingRequests
    const promise = (async () => {
        try {
            const resp = (await (
                await gotWithAuth.get('auth/info', {
                    searchParams: {
                        user_id,
                    },
                })
            ).json()) as APIResponse<UserInfo>;
            const data = resp.data;
            // 存入缓存
            userinfos[user_id] = data;
            return data;
        } finally {
            // 请求完成后（无论成功或失败），从 pendingRequests 中删除
            delete pendingRequests[user_id];
        }
    })();

    pendingRequests[user_id] = promise;
    return promise;
}

import { got, router } from './constant';
import addPresentation from './plugins/presentation';
import type { APIResponse, AuthResponse } from './types';

export async function login(username: string, totp: string): Promise<boolean> {
    const response: APIResponse<AuthResponse> = await (
        await got.post('auth/login', {
            json: {
                totp,
                username,
            },
        })
    ).json();

    if (response.status == 401) {
        addPresentation(response.message || '', 'alert');
        throw new Error('Invalid credentials');
    }
    localStorage.setItem('token', JSON.stringify(response.data));
    return true;
}

export function getLocalToken(): AuthResponse | null {
    const data = localStorage.getItem('token');
    const token: AuthResponse = data ? JSON.parse(data) : null;
    return +new Date(token?.exp_at) < +new Date() ? null : token;
}

export function getToken(redirect: boolean = true): string | undefined {
    const token = getLocalToken();
    if (redirect && token == null) {
        addPresentation('Invaild Token', 'alert');
        router.push('/login');
        throw new Error('Invaild Token');
    }
    return token?.token;
}

export async function checkToken(): Promise<boolean> {
    if (getLocalToken() == null) {
        return false;
    }
    try {
        const resp = (await (
            await got.get('auth', {
                headers: {
                    Authorization: `Bearer ${getLocalToken()?.token}`,
                },
            })
        ).json()) as APIResponse;
        return resp.status == 200;
    } catch (e) {
        localStorage.removeItem('token');
        return false;
    }
}

export async function logout() {
    localStorage.removeItem('token');
    router.push('/login');
}

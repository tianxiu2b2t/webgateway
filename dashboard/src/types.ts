export interface APIResponse<T = any> {
    status: number;
    message?: string;
    data: T;
}

export interface AuthResponse {
    token: string;
    exp_at: string;
}

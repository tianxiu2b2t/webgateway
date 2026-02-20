export interface APIResponse<T = any> {
    status: number;
    message?: string;
    data: T;
}

export interface AuthResponse {
    token: string;
    exp_at: string;
}

export interface Website {
    id: string;
    name: string;
    hosts: string[];
    ports: number[];
    certificates: string[];
    created_at: string;
    updated_at: string;
    backends: WebsiteBackend[];
    config: WebsiteConfig;
}

export interface WebsiteBackend {
    url: string;
    balance: number;
    main: boolean;
}

export interface WebsiteConfig {
    get_request_ip: WebsiteRequestIp;
}

export type WebsiteRequestIpType = 'raw' | 'proxyProtocol' | 'xForwardedFor';

export interface WebsiteRequestIp {
    type: WebsiteRequestIpType;
    data?: number;
}

export interface WebsiteCreateRequest {
    name: string;
    hosts: string[];
    ports: number[];
    certificates: string[];
    backends: WebsiteBackend[];
    config?: WebsiteConfig;
}

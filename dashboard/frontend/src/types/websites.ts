export interface WebsiteBackendInput {
    url: string;
    balance: number;
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

export interface WebsiteBackend extends WebsiteBackendInput {
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
    name?: string;
    hosts: string[];
    ports: number[];
    certificates: string[];
    backends: WebsiteBackend[];
    config?: WebsiteConfig;
}

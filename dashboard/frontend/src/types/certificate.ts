export interface CreateCertificateAuto {
    dns_provider_id: string;
    email: string;
    hostnames: string[];
}

export interface CreateCertificateManual {
    fullchain: string;
    private_key: string;
}

export type CreateCertificateContent =
    | CreateCertificateAuto
    | CreateCertificateManual;

export type CreateCertificateType = 'auto' | 'manual';

export interface Certificate {
    name?: string;
    id: string;
    created_at: string;
    updated_at: string;
    email?: string;
    expires_at: string;
    dns_provider_id?: string;
    hostnames: string[];
    fullchain?: string;
    private_key?: string;
}

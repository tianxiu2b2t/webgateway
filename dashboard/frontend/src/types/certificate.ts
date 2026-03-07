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

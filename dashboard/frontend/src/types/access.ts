export interface AccessInfo {
    total_requests: number;
    total_ips: number;
    backend_error_requests: number;
    e4xx_requests: number;
    e5xx_requests: number;
}
export interface DataQPS {
    count: number;
    time: string;
}
export interface QPS {
    count: number;
    time: Date;
}

export interface ResponseQPS {
    data: DataQPS[];
    current_time: string;
}

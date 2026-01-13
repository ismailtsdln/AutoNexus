export interface LinFrame {
    id: number;
    data: number[];
    timestamp?: number;
}

export class LinSession {
    constructor(private adapter: any) {} // Should use a LinAdapter trait/class

    async sendMasterRequest(id: number, data: number[]): Promise<void> {
        console.log(`[LIN] Sending Master Request ID: ${id.toString(16)} Data: ${data}`);
    }
}

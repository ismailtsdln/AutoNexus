import { CanAdapter } from './can.js';

export enum UdsSessionType {
    DEFAULT = 0x01,
    PROGRAMMING = 0x02,
    EXTENDED = 0x03,
    SAFETY = 0x04
}

export class UdsSession {
    constructor(
        private adapter: CanAdapter,
        private requestId: number = 0x7E0,
        private responseId: number = 0x7E8
    ) {}

    async diagnosticSessionControl(sessionType: UdsSessionType | number): Promise<number[]> {
        const frame = {
            id: this.requestId,
            data: [0x02, 0x10, sessionType, 0, 0, 0, 0, 0]
        };
        await this.adapter.send(frame);
        
        while (true) {
            const resp = await this.adapter.read();
            if (resp.id === this.responseId && resp.data) {
                if (resp.data[1] === 0x7F) {
                    throw new Error(`UDS NRC: 0x${resp.data[3]?.toString(16)}`);
                }
                if (resp.data[1] === 0x50) {
                    return resp.data;
                }
            }
        }
    }

    async readDataByIdentifier(did: number): Promise<number[]> {
        const idHigh = (did >> 8) & 0xFF;
        const idLow = did & 0xFF;
        const frame = {
            id: this.requestId,
            data: [0x03, 0x22, idHigh, idLow, 0, 0, 0, 0]
        };
        await this.adapter.send(frame);
        
        while (true) {
            const resp = await this.adapter.read();
            if (resp.id === this.responseId && resp.data && resp.data.length > 3) {
                if (resp.data[1] === 0x7F) {
                    throw new Error(`UDS NRC: 0x${resp.data[3]?.toString(16)}`);
                }
                if (resp.data[1] === 0x62) {
                    return resp.data;
                }
            }
        }
    }
}

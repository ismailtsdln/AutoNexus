import { CanAdapter } from './can';

export enum UdsSessionType {
    DEFAULT = 0x01,
    PROGRAMMING = 0x02,
    EXTENDED = 0x03,
    SAFETY = 0x04
}

export class UdsSession {
    constructor(private adapter: CanAdapter) {}

    async diagnosticSessionControl(sessionType: UdsSessionType | number): Promise<number[]> {
        const frame = {
            id: 0x7E0,
            data: [0x02, 0x10, sessionType, 0, 0, 0, 0, 0]
        };
        await this.adapter.send(frame);
        const resp = await this.adapter.read();
        return resp.data;
    }

    async readDataByIdentifier(did: number): Promise<number[]> {
        const idHigh = (did >> 8) & 0xFF;
        const idLow = did & 0xFF;
        const frame = {
            id: 0x7E0,
            data: [0x03, 0x22, idHigh, idLow, 0, 0, 0, 0]
        };
        await this.adapter.send(frame);
        const resp = await this.adapter.read();
        return resp.data;
    }
}

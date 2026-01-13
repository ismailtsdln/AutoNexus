export interface CanFrame {
    id: number;
    data: number[];
    isExtended?: boolean;
    isFd?: boolean;
    timestamp?: number;
}

export abstract class CanAdapter {
    abstract open(): Promise<void>;
    abstract close(): Promise<void>;
    abstract send(frame: CanFrame): Promise<void>;
    abstract read(): Promise<CanFrame>;
    abstract isOpen(): boolean;
}

export class MockCanAdapter extends CanAdapter {
    private isOpenStatus: boolean = false;

    async open(): Promise<void> {
        this.isOpenStatus = true;
    }

    async close(): Promise<void> {
        this.isOpenStatus = false;
    }

    async send(frame: CanFrame): Promise<void> {
        if (!this.isOpenStatus) throw new Error("Adapter not open");
        console.log(`[MockAdapter] Sending CAN frame ID: ${frame.id.toString(16)} Data: ${frame.data}`);
    }

    async read(): Promise<CanFrame> {
        if (!this.isOpenStatus) throw new Error("Adapter not open");
        return {
            id: 0x7E8,
            data: [0x03, 0x50, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
            timestamp: Date.now()
        };
    }

    isOpen(): boolean {
        return this.isOpenStatus;
    }
}

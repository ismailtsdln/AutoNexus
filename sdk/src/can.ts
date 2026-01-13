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
}

export class MockCanAdapter extends CanAdapter {
    private isOpen: boolean = false;

    async open(): Promise<void> {
        this.isOpen = true;
    }

    async close(): Promise<void> {
        this.isOpen = false;
    }

    async send(frame: CanFrame): Promise<void> {
        if (!this.isOpen) throw new Error("Adapter not open");
        console.log(`[MockAdapter] Sending CAN frame ID: ${frame.id.toString(16)} Data: ${frame.data}`);
    }

    async read(): Promise<CanFrame> {
        if (!this.isOpen) throw new Error("Adapter not open");
        return {
            id: 0x123,
            data: [0x11, 0x22, 0x33],
            timestamp: Date.now()
        };
    }
}

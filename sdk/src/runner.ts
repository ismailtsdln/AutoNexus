import { NodeVM } from 'vm2';
import * as AutoNexus from './index';

export interface ScriptResult {
    success: boolean;
    output: string[];
    error?: string;
}

export class ScriptRunner {
    private vm: NodeVM;
    private logs: string[] = [];

    constructor() {
        this.vm = new NodeVM({
            console: 'redirect',
            sandbox: {
                AutoNexus: AutoNexus
            },
            require: {
                external: true,
                builtin: ['*'],
                root: './',
            }
        });

        const handleLog = (type: string, ...args: any[]) => {
            this.logs.push(`[${type}] ${args.join(' ')}`);
        };

        // Use casting if types are missing for .on() or options
        const vmAny = this.vm as any;
        if (vmAny.on) {
            vmAny.on('console.log', (...args: any[]) => handleLog('LOG', ...args));
            vmAny.on('console.error', (...args: any[]) => handleLog('ERR', ...args));
        }
    }

    async run(code: string): Promise<ScriptResult> {
        this.logs = [];
        try {
            // Wrap in async to support top-level await if needed, or just run normally
            await this.vm.run(code, 'script.js');
            return {
                success: true,
                output: this.logs
            };
        } catch (e: any) {
            return {
                success: false,
                output: this.logs,
                error: e.message
            };
        }
    }
}

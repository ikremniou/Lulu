import { DisassemblyToken } from "../../../types/disassembly-token";

export class TopLevelFunction implements DisassemblyToken {
    readonly sizeInBytes: number;

    constructor() {

    }

    write(buffer: Buffer): void {
        throw new Error("Method not implemented.");
    }

    toString(): string {
        throw new Error("Method not implemented.");
    }
}
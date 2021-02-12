export interface Disassembly {
}

export interface DisassemblyToken {
    readonly sizeInBytes: number;
    write(buffer: Buffer): void;
    toString(): string;
}
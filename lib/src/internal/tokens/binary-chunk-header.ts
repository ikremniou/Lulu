import { DisassemblyToken } from '../../../types/disassembly-token';
import {
    BinaryChunkHeaderEndianness,
    BinaryChunkHeaderFormat,
    BinaryChunkHeaderIntegralFlag,
    BinaryChunkHeaderSignature,
    BinaryChunkHeaderSizeOfInstruction,
    BinaryChunkHeaderSizeOfInt,
    BinaryChunkHeaderSizeOfLuaNumber,
    BinaryChunkHeaderSizeOfSizeT,
    BinaryChunkHeaderVersion,
} from '../disassembly/enum/header';

enum BinaryOffsets {
    Signature = 0,
    Version = 4,
    Format = 5,
    Endianness = 6,
    SizeOfInt = 7,
    SizeOfSizeT = 8,
    SizeOfInstruction = 9,
    SizeOfLuaNumber = 10,
    InternalFlag = 11,
    Header = 12,
}

export class BinaryChunkHeader implements DisassemblyToken {
    readonly sizeInBytes: number;
    readonly signature: BinaryChunkHeaderSignature;
    readonly version: BinaryChunkHeaderVersion;
    readonly format: BinaryChunkHeaderFormat;
    readonly endianness: BinaryChunkHeaderEndianness;
    readonly sizeOfInt: BinaryChunkHeaderSizeOfInt;
    readonly sizeOfSizeT: BinaryChunkHeaderSizeOfSizeT;
    readonly sizeOfInstruction: BinaryChunkHeaderSizeOfInstruction;
    readonly sizeOfLuaNumber: BinaryChunkHeaderSizeOfLuaNumber;
    readonly integralFlag: BinaryChunkHeaderIntegralFlag;

    constructor(rawFile: Buffer) {
        this.sizeInBytes = BinaryOffsets.Header;
        this.signature = rawFile.readInt32BE(BinaryOffsets.Signature);
        if (this.signature !== BinaryChunkHeaderSignature.Lua)
            throw new Error(`Invalid header Signature: ${this.signature}`);

        this.version = rawFile.readInt8(BinaryOffsets.Version);
        if (this.version !== BinaryChunkHeaderVersion.Lua51)
            throw new Error(`Not supported Lua version: ${this.version}`);

        this.format = rawFile.readInt8(BinaryOffsets.Format);
        if (this.format !== BinaryChunkHeaderFormat.Official)
            throw new Error(`Unsupported Format version: ${this.format}`);

        this.endianness = rawFile.readInt8(BinaryOffsets.Endianness);
        if (
            this.endianness !== BinaryChunkHeaderEndianness.BigEndian &&
            this.endianness !== BinaryChunkHeaderEndianness.LittleEndian
        )
            throw new Error(`Unsupported Endianness: ${this.endianness}`);

        this.sizeOfInt = rawFile.readInt8(BinaryOffsets.SizeOfInt);
        if (this.sizeOfInt != BinaryChunkHeaderSizeOfInt.Default)
            throw new Error(`Unsupported Int size: ${this.sizeOfInt}`);

        this.sizeOfSizeT = rawFile.readInt8(BinaryOffsets.SizeOfSizeT);
        if (this.sizeOfSizeT !== BinaryChunkHeaderSizeOfSizeT.Default)
            throw new Error(`Unsupported SizeT: ${this.sizeOfSizeT}`);

        this.sizeOfInstruction = rawFile.readInt8(BinaryOffsets.SizeOfInstruction);
        if (this.sizeOfInstruction !== BinaryChunkHeaderSizeOfInstruction.Default)
            throw new Error(`Unsupported Instruction size: ${this.sizeOfInstruction}`);

        this.sizeOfLuaNumber = rawFile.readUInt32LE(BinaryOffsets.SizeOfLuaNumber);
        if (this.sizeOfLuaNumber !== BinaryChunkHeaderSizeOfLuaNumber.Default)
            throw new Error(`Unsupported Lua number: ${this.sizeOfLuaNumber}`);

        this.integralFlag = rawFile.readUInt32LE(BinaryOffsets.InternalFlag);
        if (
            this.integralFlag !== BinaryChunkHeaderIntegralFlag.FloatingPoint &&
            this.integralFlag !== BinaryChunkHeaderIntegralFlag.IntegralNumber
        )
            throw new Error(`Unsupported Integral: ${this.integralFlag}`);
    }

    public write(buffer: Buffer): void {
        buffer.writeUInt32BE(this.signature, BinaryOffsets.Signature);
        buffer.writeInt8(this.version, BinaryOffsets.Version);
        buffer.writeInt8(this.format, BinaryOffsets.Format);
        buffer.writeInt8(this.endianness, BinaryOffsets.Endianness);
        buffer.writeInt8(this.sizeOfInt, BinaryOffsets.SizeOfInt);
        buffer.writeInt8(this.sizeOfSizeT, BinaryOffsets.SizeOfSizeT);
        buffer.writeInt8(this.sizeOfInstruction, BinaryOffsets.SizeOfInstruction);
        buffer.writeInt8(this.sizeOfLuaNumber, BinaryOffsets.SizeOfLuaNumber);
        buffer.writeInt8(this.integralFlag, BinaryOffsets.InternalFlag);
    }

    public toString(): string {
        throw new Error('Method not implemented.');
    }
}

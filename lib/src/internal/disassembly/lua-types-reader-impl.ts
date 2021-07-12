import { BinaryChunkHeader } from '../tokens/binary-chunk-header';
import { CursorPosition } from './cursor-position';
import { BinaryChunkHeaderEndianness } from './enum/header';
import { LuaTypesReader } from './lua-types-reader';

export class LuaTypesReaderImpl implements LuaTypesReader {
    constructor(private readonly header: BinaryChunkHeader) {}


    public readString(cursor: CursorPosition): string | undefined {
        const stringSize = this.readSizeT(cursor);
        if (stringSize == 0) {
            return undefined;
        }

        const stringBuffer = cursor.buffer.slice(cursor.offset, cursor.offset + stringSize);
        cursor.offset += stringSize;
        return stringBuffer.toString();
    }

    public readInt(cursor: CursorPosition): number {
       let integer: number = 0;
       if (this.header.endianness == BinaryChunkHeaderEndianness.BigEndian) {
        integer = cursor.buffer.readInt32BE(cursor.offset);
       } else {
        integer = cursor.buffer.readInt32LE(cursor.offset);
       }
       cursor.offset += this.header.sizeOfInt;
       return integer;
    }

    public readSizeT(cursor: CursorPosition): number {
        let sizeT: number = 0;
        if (this.header.endianness == BinaryChunkHeaderEndianness.BigEndian) {
            sizeT = cursor.buffer.readUInt32BE(cursor.offset);
        } else {
            sizeT = cursor.buffer.readUInt32LE(cursor.offset);
        }
        cursor.offset += this.header.sizeOfSizeT;
        return sizeT;
    }

    public readByte(cursor: CursorPosition): number {
        const byte = cursor.buffer.readInt8(cursor.offset);
        cursor.offset++;
        return byte;
    }


    public readInstruction(cursor: CursorPosition): number {
        let instruction: number = 0;
        if (this.header.endianness == BinaryChunkHeaderEndianness.BigEndian) {
            instruction = cursor.buffer.readUInt32BE(cursor.offset);
        } else {
            instruction = cursor.buffer.readUInt32LE(cursor.offset);
        }
        cursor.offset += this.header.sizeOfSizeT;
        return instruction;
    }
}

import { CursorPosition } from './cursor-position';

export interface LuaTypesReader {
    readInstruction(cursor: CursorPosition): number;
    readByte(cursor: CursorPosition): number;
    readInt(cursor: CursorPosition): number;
    readString(cursor: CursorPosition): string | undefined;
    readSizeT(cursor: CursorPosition): number;
}

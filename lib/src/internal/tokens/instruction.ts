import { CursorPosition } from "../disassembly/cursor-position";
import { LuaTypesReader } from "../disassembly/lua-types-reader";

export class Instruction {
    private _value: number;

    constructor(private readonly _typesReader: LuaTypesReader) {}

    public from(cursor: CursorPosition): void {
        this._value = this._typesReader.readInstruction(cursor);

    } 
}
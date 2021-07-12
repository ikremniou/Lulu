import { CursorPosition } from '../disassembly/cursor-position';
import { LuaTypesReader } from '../disassembly/lua-types-reader';

export class InstructionList {
    private _listSize!: number;
    private _instructions: Instruction;

    constructor(private readonly _typesReader: LuaTypesReader) {}

    public from(cursor: CursorPosition) {
        this._listSize = this._typesReader.readInt(cursor);
    }
}

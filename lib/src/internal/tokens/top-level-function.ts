import { DisassemblyToken } from '../../../types/disassembly-token';
import { CursorPosition } from '../disassembly/cursor-position';
import { LuaTypesReader } from '../disassembly/lua-types-reader';
import { IsVarArg } from '../disassembly/enum/is-var-arg';



export class TopLevelFunction implements DisassemblyToken {
    readonly sizeInBytes!: number;

    private _sourceName?: string;
    private _lineDefined!: number;
    private _lastLineDefined!: number;
    private _numberOfUpValues!: number;
    private _numberOfParameters!: number;
    private _isVarArg!: IsVarArg;
    private _maxStackSize!: number;
    private _instructionList: InstructionList;

    constructor(private readonly _typesReader: LuaTypesReader) {}

    public from(cursor: CursorPosition) {
        this._sourceName = this._typesReader.readString(cursor);
        this._lineDefined = this._typesReader.readInt(cursor);
        this._lastLineDefined = this._typesReader.readInt(cursor);
        this._numberOfUpValues = this._typesReader.readByte(cursor);
        this._numberOfParameters = this._typesReader.readByte(cursor);
        this._isVarArg = this._typesReader.readByte(cursor);
        this._maxStackSize = this._typesReader.readByte(cursor);

    }

    write(_buffer: Buffer): void {
        throw new Error('Method not implemented.');
    }

    toString(): string {
        throw new Error('Method not implemented.');
    }
}

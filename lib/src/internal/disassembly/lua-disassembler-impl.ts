import { Disassembly } from '../../../types/disassembly-token';
import { BinaryChunkHeader } from '../tokens/binary-chunk-header';
import { TopLevelFunction } from '../tokens/top-level-function';
import { CursorPosition } from './cursor-position';
import { LuaDisassembler } from './lua-disassembler';
import { LuaTypesReaderImpl } from './lua-types-reader-impl';

export class LuaDisassemblerImpl implements LuaDisassembler {
    private _internalBuffer!: Buffer;
    private _bufferOffset!: number;

    constructor(private readonly _binaryHeader: BinaryChunkHeader,
        private readonly _cursor: CursorPosition) {}

    public disassemble(): Disassembly {
        const typesReader = new LuaTypesReaderImpl(this._binaryHeader);
        const topLevelFunction = new TopLevelFunction(typesReader);
        topLevelFunction.from(this._cursor);

        return {} as any;
    }
}

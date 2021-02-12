import { Disassembly } from "../../../types/disassembly-token";
import { BinaryChunkHeader } from "../tokens/binary-chunk-header";
import { LuaDisassembler } from "./lua-disassembler";

export class LuaDisassemblerImpl implements LuaDisassembler {
    private _internalBuffer: Buffer;
    private _bufferOffset: number;

    constructor(private readonly _binaryHeader: BinaryChunkHeader) { }

    public disassemble(rawFile: Buffer): Disassembly {
        this._internalBuffer = rawFile.slice(this._binaryHeader.sizeInBytes);
        


        throw new Error("Method not implemented.");
    }

}
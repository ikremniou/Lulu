import { BinaryChunkHeader } from "./tokens/binary-chunk-header";
import { DisassemblerFactory } from "./disassembly-factory";
import { LuaDisassembler } from "./disassembly/lua-disassembler";
import { LuaDisassemblerImpl } from "./disassembly/lua-disassembler-impl";

export class DisassemblerFactoryImpl implements DisassemblerFactory {
    public createDisassembler(rawFile: Buffer): LuaDisassembler {
        const luaHeader = new BinaryChunkHeader(rawFile);
        // Select the Disassembler depending on the header
        return new LuaDisassemblerImpl(luaHeader);
    }
}
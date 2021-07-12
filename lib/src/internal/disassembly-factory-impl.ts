import { BinaryChunkHeader } from './tokens/binary-chunk-header';
import { DisassemblerFactory } from './disassembly-factory';
import { LuaDisassembler } from './disassembly/lua-disassembler';
import { LuaDisassemblerImpl } from './disassembly/lua-disassembler-impl';
import { CursorPosition } from './disassembly/cursor-position';

export class DisassemblerFactoryImpl implements DisassemblerFactory {
    public createDisassembler(rawFile: Buffer): LuaDisassembler {
        const luaHeader = new BinaryChunkHeader(rawFile);
        // Select the Disassembler depending on the header
        const cursor: CursorPosition = { buffer: rawFile, offset: luaHeader.sizeInBytes };
        return new LuaDisassemblerImpl(luaHeader, cursor);
    }
}

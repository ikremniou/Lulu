import { LuaDisassembler } from "./disassembly/lua-disassembler";

/** @internal */
export interface DisassemblerFactory {
    createDisassembler(rawFile: Buffer): LuaDisassembler;
}
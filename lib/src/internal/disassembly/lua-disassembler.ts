import { Disassembly } from "../../../types/disassembly-token";

/** @internal */
export interface LuaDisassembler {
    disassemble(rawFile: Buffer): Disassembly;
}
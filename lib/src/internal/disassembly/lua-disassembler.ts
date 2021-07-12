import { Disassembly } from "../../../types/disassembly-token";
import { CursorPosition } from "./cursor-position";

/** @internal */
export interface LuaDisassembler {
    disassemble(): Disassembly;
}
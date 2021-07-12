import { DisassemblerFactory } from "./internal/disassembly-factory"
import { DisassemblerFactoryImpl } from "./internal/disassembly-factory-impl";

const disassemblerFactory: DisassemblerFactory = new DisassemblerFactoryImpl();

export function disassemblyFromBuffer(rawFile: Buffer) {
    const disassembler = disassemblerFactory.createDisassembler(rawFile); 
    disassembler.disassemble();
}
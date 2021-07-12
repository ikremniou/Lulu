export enum BinaryChunkHeaderSignature {
    Lua = 0x1B4C7561
}

export enum BinaryChunkHeaderVersion {
    Lua51 = 0x51
}

export enum BinaryChunkHeaderFormat {
    Official = 0
}

export enum BinaryChunkHeaderEndianness {
    BigEndian = 0,
    LittleEndian = 1
}

export enum BinaryChunkHeaderSizeOfInt {
    Default = 4
}

export enum BinaryChunkHeaderSizeOfSizeT {
    Default = 4
}

export enum BinaryChunkHeaderSizeOfInstruction {
    Default = 4
}

export enum BinaryChunkHeaderSizeOfLuaNumber {
    Default = 8
}

export enum BinaryChunkHeaderIntegralFlag {
    FloatingPoint = 0,
    IntegralNumber = 1
}
import fs from 'fs';
import path from 'path'
import { disassemblyFromBuffer } from '.';

fs.readFile(path.join(__dirname, '..', '..', 'assets', 'config.lu'), (_, data) => {
    disassemblyFromBuffer(data);
});
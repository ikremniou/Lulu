import * as fs from 'fs/promises';
import * as path from 'path';
import { disassemblyFromBuffer } from '../src';

describe('Assets Decompilation', () => {
    it('Should decompile all of the assets successfully', async () => {
        const assetsPath = path.resolve('assets');
        const files = await fs.readdir(assetsPath);
        for (const fileName of files) {
            const rawFile = await fs.readFile(path.join(assetsPath, fileName));
            disassemblyFromBuffer(rawFile);
        }
    });
});

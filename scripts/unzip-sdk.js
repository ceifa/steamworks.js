const path = require('node:path')
const sevenBin = require('7zip-bin');
const { extractFull } = require('node-7z');

const password = process.argv[2];
if (!password) {
    throw new Error('Password is required');
}

const root = path.join(__dirname, '..')
extractFull(path.join(root, 'encrypted-sdk.zip'), root, {
    $bin: sevenBin.path7za,
    password
})
const path = require('node:path')
const sevenBin = require('7zip-bin');
const { extractFull, add } = require('node-7z');

const password = process.argv[2];
if (!password) {
    throw new Error('Password is required');
}

const root = path.join(__dirname, '..')
extractFull(path.join(root, 'steamworks-sdk.zip'), root, {
    $bin: sevenBin.path7za
})

add(path.join(root, 'encrypted-sdk.zip'), path.join(root, 'sdk'), {
    $bin: sevenBin.path7za,
    recursive: true,
    password
})
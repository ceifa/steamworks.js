const path = require('node:path');
const fs = require('node:fs');
const child_process = require('node:child_process');

const copy = (source, dest) => {
    try { fs.mkdirSync(path.dirname(dest), { recursive: true }) } catch { }
    fs.copyFileSync(source, dest)
}

const { platform, arch } = process

let files = []
let folder = undefined

if (platform === 'win32' && arch === 'x64') {
    folder = 'win64'
    files = ['steam_api64.dll', 'steam_api64.lib']
} else if (platform === 'linux' && arch === 'x64') {
    folder = 'linux64'
    files = ['libsteam_api.so']
} else if (platform === 'darwin') {
    folder = 'osx'
    files = ['libsteam_api.dylib']
} else {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

const dist = path.join(__dirname, 'dist', folder)
const redist = path.join(__dirname, 'sdk/redistributable_bin', folder)
files.forEach(file => copy(path.join(redist, file), path.join(dist, file)))

const relative = path.relative(process.cwd(), dist)
const params = [
    'build',
    '--platform',
    '--no-dts-header',
    '--js', 'false',
    '--dts', '../../client.d.ts',
    relative,
    process.argv.slice(2).join(' ')
]

child_process.spawn('napi', params, { stdio: 'inherit', shell: true })
    .on('exit', err => {
        if (err) {
            throw err;
        }
    })

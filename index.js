const { platform, arch } = process

/** @typedef {typeof import('./client.d')} Client */
/** @type {Client | undefined} */
let nativeBinding = undefined

if (platform === 'win32' && arch === 'x64') {
    nativeBinding = require('./dist/win64/steamworksjs.win32-x64-msvc.node')
} else if (platform === 'linux' && arch === 'x64') {
    nativeBinding = require('./dist/linux64/steamworksjs.linux-x64-gnu.node')
} else if (platform === 'darwin') {
    nativeBinding = require('./dist/osx/steamworksjs.darwin-x64.node')
} else {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

/**
 * Initialize the steam client or throw an error if it fails
 * @param {number | undefined} appId - App ID of the game to load, if undefined, will search for a steam_appid.txt file
 * @returns {Omit<Client, 'init' | 'runCallbacks'>}
*/
module.exports.init = (appId) => {
    if (!appId) {
        try {
            const content = require('fs').readFileSync('steam_appid.txt', 'utf8')
            if (content) {
                appId = parseInt(content)
            } else {
                throw new Error('steam_appid.txt file is not valid')
            }
        } catch (e) {
            throw new Error('Failed to load steam_appid.txt file')
        }
    }

    const { init: internalInit, runCallbacks, ...api } = nativeBinding

    internalInit(appId)
    setInterval(runCallbacks, 1000 / 30)

    return api
}
const { platform, arch } = process

/** @typedef {typeof import('./client.d')} Client */
/** @type {Client | null} */
let nativeBinding = null

switch (platform) {
    case 'win32':
        switch (arch) {
            case 'x64':
                try {
                    nativeBinding = require('./dist/steamworksjs.win32-x64-msvc.node')
                } catch (e) {
                    loadError = e
                }
                break
            case 'ia32':
                try {
                    nativeBinding = require('./dist/steamworksjs.win32-ia32-msvc.node')
                } catch (e) {
                    loadError = e
                }
                break
            case 'arm64':
                try {
                    nativeBinding = require('./dist/steamworksjs.win32-arm64-msvc.node')
                } catch (e) {
                    loadError = e
                }
                break
            default:
                throw new Error(`Unsupported architecture on Windows: ${arch}`)
        }
        break
    case 'darwin':
        switch (arch) {
            case 'x64':
                try {
                    nativeBinding = require('./dist/steamworksjs.darwin-x64.node')
                } catch (e) {
                    loadError = e
                }
                break
            case 'arm64':
                try {
                    nativeBinding = require('./dist/steamworksjs.darwin-arm64.node')
                } catch (e) {
                    loadError = e
                }
                break
            default:
                throw new Error(`Unsupported architecture on macOS: ${arch}`)
        }
        break
    case 'linux':
        switch (arch) {
            case 'x64':
                try {
                    nativeBinding = require('./dist/steamworksjs.linux-x64-gnu.node')
                } catch (e) {
                    loadError = e
                }

                break
            default:
                throw new Error(`Unsupported architecture on Linux: ${arch}`)
        }
        break
    default:
        throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
    if (loadError) {
        throw loadError
    }
    throw new Error(`Failed to load native binding`)
}
/**
 * Initialize the steam client or throw an error if it fails
 * @param {number | undefined} appId - App ID of the game to load, if undefined, will search for a steam_appid.txt file
 * @returns {Client}
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
    setInterval(runCallbacks, 50)

    return api
}
import { SteamClient } from '../dist/index.js'

await SteamClient.init(480)
console.log(await SteamClient.getSteamId())

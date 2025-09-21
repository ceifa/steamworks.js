const { init } = require('../index.js')

const client = init(480)
const steamId = client.localplayer.getSteamId().steamId64
console.log('My Steam Id: ' + steamId)
console.log(`My name: ${client.friends.getPersonaName(steamId)}`)
process.exit(0)

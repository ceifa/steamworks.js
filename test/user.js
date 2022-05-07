const { init } = require('../index.js')

const client = init(480)
console.log('Name: ' + client.localplayer.getName())
console.log('Level: ' + client.localplayer.getLevel())
console.log('Ip Country: ' + client.localplayer.getIpCountry())
console.log('Steam Id: ' + JSON.stringify(client.localplayer.getSteamId()))

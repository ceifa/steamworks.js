const { init } = require('./index.js')

const client = init(480)
console.log(client.localplayer.getName())
process.env.IS_TESTING = true

const { init } = require('../index.js')

const client = init(480)

client.networking_utils.initRelayNetworkAccess();

setInterval(() => {
  console.log(client.networking_utils.detailedRelayNetworkStatus())
}, 1000)


const mySteamId = client.localplayer.getSteamId().steamId64;
setTimeout(() => {
  setInterval(() => {
    client.networking_messages.sendMessageToUser(mySteamId, 1, Buffer.from("Hello, from client!"), 0);
  }, 1000)
}, 5000)


//client.networking_messages.sendMessageToUser(mySteamId, 0, Buffer.from("Hello, world!"), 0);

// client.networking_messages.sendMessageToUser(mySteamId, 1, Buffer.from("Hello, world!"), 0);
//client.networking_messages.sendMessageToUser(mySteamId, 3, Buffer.from("Hello, world!"), 0);
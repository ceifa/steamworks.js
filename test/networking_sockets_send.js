process.env.IS_TESTING = true

const { init } = require('../index.js')

const client = init(480, true)

client.networking_utils.initRelayNetworkAccess();

const socket = 96;

// we shouldn't need to listen for p2p requests since we're sending it

const mySteamId = client.localplayer.getSteamId().steamId64;
console.log(mySteamId);

// now actually listen for new messages
/*
setInterval(() => {
  let messages = client.networking_sockets.receiveP2PMessages(10); // 10 _from each_ connection
  messages.forEach(message => console.log(message));
}, 1000 / 60);
*/

// now let's send a connection request to the server
client.networking_sockets.connectP2P(mySteamId, 0);

client.networking_sockets.sendP2PMessage(mySteamId, Buffer.from("Hello, from client!"), 1);
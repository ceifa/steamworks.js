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
    client.networking_messages.sendMessageToUser(mySteamId, 1, Buffer.from("Hello, from client!"), 1);
  }, 1000)
}, 5000)

setInterval(() => {
  let messages = []
  try {
    messages = client.networking_messages.receiveMessagesOnChannel(0);

    while(messages.length > 0){
      const message = messages.shift();
      console.log("Received message")
      console.log(message?.steamId)
      console.log(message?.data.toString());

      client.networking_messages.sendMessageToUser(message.steamId.steamId64, 1, Buffer.from("Hello, world!"), 1);
      console.log("Sent message")
    }

  } catch (e) { 
    console.error(e)
  }
}, 1000 / 60)


//client.networking_messages.sendMessageToUser(mySteamId, 0, Buffer.from("Hello, world!"), 0);

// client.networking_messages.sendMessageToUser(mySteamId, 1, Buffer.from("Hello, world!"), 0);
//client.networking_messages.sendMessageToUser(mySteamId, 3, Buffer.from("Hello, world!"), 0);
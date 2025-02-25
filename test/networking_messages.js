const { init } = require('../index.js')

const client = init(480)

client.networking_utils.initRelayNetworkAccess();

const mySteamId = client.localplayer.getSteamId().steamId64;

setInterval(() => {
  let messages = []
  try {
    do {
      messages = client.networking_messages.receiveMessagesOnChannel(0);

      while(messages.length > 0){
        const message = messages.shift();
        console.log("Received message")
        console.log(message?.steamId)
        console.log(message?.data.toString());
      }
    } while(messages.length > 0)
  } catch (e) { 
    if(messages.length > 0){
      console.error("Error receiving messages")
      console.error(e)
    }
  }
}, 1000 / 60)

client.networking_messages.sendMessageToUser(mySteamId, 0, Buffer.from("Hello, world!"), 0);
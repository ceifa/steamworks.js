process.env.IS_TESTING = true

const { init } = require('../index.js')

const client = init(480, true)

client.networking_utils.initRelayNetworkAccess();

setInterval(() => {
  console.log(client.networking_utils.detailedRelayNetworkStatus())
}, 1000)

const mySteamId = client.localplayer.getSteamId().steamId64;

setInterval(() => {
  let messages = []
  try {
    messages = client.networking_messages.receiveMessagesOnChannel(1);

    while(messages.length > 0){
      const message = messages.shift();
      console.log("Received message")
      console.log(message?.steamId)
      console.log(message?.data.toString());

      // client.networking_messages.sendMessageToUser(mySteamId, 1, Buffer.from("Hello from server!"), 0);
      // console.log("Sent message")
    }

  } catch (e) { 
    console.error(e)
  }
}, 1000 / 60)

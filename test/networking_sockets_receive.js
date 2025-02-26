process.env.IS_TESTING = true

const { init } = require('../index.js')

const client = init(480, true)

client.networking_utils.initRelayNetworkAccess();

const socket = 0;
const localSocket = 6969;

client.networking_sockets.createListenSocketP2P(socket);
client.networking_sockets.createListenSocketIP(localSocket);

// enable p2p connections
client.networking_sockets.setAmIServer(true);

// set up one listener just to process the listen p2p events
setInterval(() => {
  console.log("Processing listen p2p events");
  client.networking_sockets.processListenP2PEvents();
}, 1000 / 60)

// and the local ip guh
setInterval(() => {
  console.log("Processing listen ip events");
  client.networking_sockets.processListenIPEvents();
}, 1000 / 60)


// now actually listen for new messages
setInterval(() => {
  console.log("processing p2p messages");
  let messages = client.networking_sockets.receiveP2PMessages(10); // 10 _from each_ connection
  messages.forEach(message => console.log(message));
}, 1000 / 60);
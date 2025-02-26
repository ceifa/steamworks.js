process.env.IS_TESTING = true

const { init } = require('../index.js')

const client = init(480, true)

client.networking_utils.initRelayNetworkAccess();

const socket = 0;

client.networking_sockets.createListenSocketP2P(socket);

// enable p2p connections
client.networking_sockets.setAcceptNewP2PRequests(true);
// set up one listener just to process the listen p2p events
setInterval(() => {
  console.log("Processing listen p2p events");
  client.networking_sockets.processListenP2PEvents();
}, 1000 / 60)


// now actually listen for new messages
setInterval(() => {
  let messages = client.networking_sockets.receiveP2PMessages(10); // 10 _from each_ connection
  messages.forEach(message => console.log(message));
}, 1000 / 60);
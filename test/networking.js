const rl = require('readline');
const { init, SteamCallback } = require('../index.js')

const client = init(480)
client.callback.register(SteamCallback.P2PSessionRequest, ({ remote }) => {
    console.log(`P2PSessionRequest from ${remote}`)
    client.networking.acceptP2PSession(remote)
})

client.callback.register(SteamCallback.P2PSessionConnectFail, ({ remote, error }) => {
    console.log(`Failed to connect to ${remote} with error ${error}`)
})

const rlInterface = rl.createInterface({
    input: process.stdin,
    output: process.stdout
})

rlInterface.question('Enter a lobby id or press enter to create one: ', async lobbyId => {
    let lobby
    if (lobbyId) {
        lobby = await client.matchmaking.joinJobby(BigInt(lobbyId))
        lobby.getMembers().forEach(peer => {
            client.networking.sendP2PPacket(peer.steamId64, client.networking.SendType.Reliable, Buffer.from('Connection request'))
        })
    } else {
        lobby = await client.matchmaking.createLobby(client.matchmaking.LobbyType.Public, 10)
        console.log(`Created lobby with id ${lobby.id}`)
    }

    let askChatMessage
    askChatMessage = () => {
        rlInterface.question(client.localplayer.getName() + ': ', line => {
            const message = client.localplayer.getName() + ': ' + line
            lobby.getMembers().forEach(peer => {
                client.networking.sendP2PPacket(peer.steamId64, client.networking.SendType.Reliable, Buffer.from(message))
            })
            askChatMessage()
        })
    }
    askChatMessage()

    setInterval(() => {
        let size;
        while ((size = client.networking.isP2PPacketAvailable()) > 0) {
            const { steamId, data } = client.networking.readP2PPacket(size)
            if (steamId.steamId64 !== client.localplayer.getSteamId().steamId64) {
                console.log(data.toString() + '\n')
            }
        }
    }, 66);
})
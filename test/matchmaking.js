const { init, SteamCallback } = require('../index.js')

const client = init(480)
const callback = client.callback.register(SteamCallback.LobbyDataUpdate, (batata) => {
    console.log(batata)
});

setTimeout(() => {
    callback.disconnect()
}, 5000);

(async () => {
    const lobby = await client.matchmaking.createLobby(client.matchmaking.LobbyType.Public, 2)
    console.log(lobby.id)

    lobby.setData('batata', '1')
    lobby.mergeFullData({
        'hello': 'world',
        'batata': '2'
    })
    console.log(lobby.getFullData())

    console.log("=====")
    console.log(lobby.getData('batata'))

    lobby.leave();

    console.log("=====")
    const lobbies = await client.matchmaking.getLobbies();
    console.log(lobbies.map(lobby => lobby.id))
})();
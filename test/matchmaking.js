const { init, SteamCallback } = require('../index.js')

const client = init(480)
const callback1 = client.callback.register(SteamCallback.LobbyDataUpdate, (data) => {
    console.log('LobbyDataUpdate', data)
});

const callback2 = client.callback.register(SteamCallback.LobbyChatUpdate, (data) => {
    console.log('LobbyChatUpdate', data)
});

setTimeout(() => {
    callback1.disconnect()
    callback2.disconnect()
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
    const lobbies = await client.matchmaking.getLobbies()
    console.log(lobbies.map(lobby => lobby.id))

    const lobbyWithMorePeople = await lobbies.sort((a, b) => Number(b.getMemberCount() - a.getMemberCount()))[1].join()
    console.log("Joined at " + lobbyWithMorePeople.id + " with " + lobbyWithMorePeople.getMemberCount() + " members:")
    console.log(lobbyWithMorePeople.getMembers())
})();
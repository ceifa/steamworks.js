const { init, SteamCallback } = require('../index.js')

const client = init(480)
const callback1 = client.callback.register(SteamCallback.LobbyDataUpdate, (data) => {
    console.log('LobbyDataUpdate', data)
});

const callback2 = client.callback.register(SteamCallback.LobbyChatUpdate, (data) => {
    console.log('LobbyChatUpdate', data)
});

let chatLobby = undefined
const callback3 = client.callback.register(SteamCallback.LobbyChatMsg, (data) => {
    const text = chatLobby
        ? chatLobby.getChatEntry(data.chat_id).toString('utf8')
        : '(lobby not held)'
    console.log('LobbyChatMsg', data, text)
});

setTimeout(() => {
    callback1.disconnect()
    callback2.disconnect()
    callback3.disconnect()
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

    // A message you send is relayed back to you as a LobbyChatMsg callback.
    chatLobby = lobby
    lobby.sendChatMessage(Buffer.from('hello lobby', 'utf8'))
    await new Promise(resolve => setTimeout(resolve, 1000))
    chatLobby = undefined

    lobby.leave();

    console.log("=====")
    const lobbies = await client.matchmaking.getLobbies()
    console.log(lobbies.map(lobby => lobby.id))

    const lobbyWithMorePeople = await lobbies.sort((a, b) => Number(b.getMemberCount() - a.getMemberCount()))[1].join()
    console.log("Joined at " + lobbyWithMorePeople.id + " with " + lobbyWithMorePeople.getMemberCount() + " members:")
    console.log(lobbyWithMorePeople.getMembers())
})();
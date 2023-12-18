const { init } = require('../index.js')

const client = init(480)

;(async () => {
    {
        const ticket = await client.auth.getSessionTicketWithSteamId(client.localplayer.getSteamId().steamId64)

        console.log('getSessionTicketWithSteamId: ', ticket.getBytes())

        ticket.cancel()
    }

    {
        const ticket = await client.auth.getAuthTicketForWebApi('test')

        console.log('getAuthTicketForWebApi: ', ticket.getBytes())

        ticket.cancel()
    }
})()

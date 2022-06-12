const { init } = require('../index.js')

const client = init(480);

(async () => {
    const ticket = await client.auth.getSessionTicket()

    console.log("Ticket: ", ticket.getBytes())

    ticket.cancel()
})()
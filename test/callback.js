const { init, SteamCallback } = require('../index.js')

const client = init(480);

const handle = client.callback.register(SteamCallback.PersonaStateChange, (value) => {
    console.log(value)
})

setTimeout(() => {
    handle.disconnect()
}, 3000);
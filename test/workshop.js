const { init } = require('../index.js');

(async () => {
    const client = init(4000)

    const items = client.workshop.getSubscribedItems()
    console.log(`${items.length} subscribed items: ${items.join(', ')}`)

    const details = await client.workshop.getItems(items)
    console.log(details)
})()

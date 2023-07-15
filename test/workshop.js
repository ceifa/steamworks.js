const { init } = require('../index.js')

const client = init(4000)
const items = client.workshop.getSubscribedItems()

console.log(`${items.length} subscribed items: ${items.join(', ')}`)
// A signed ownership proof, end to end on one account:
//   GetItemsByID -> SerializeResult -> DeserializeResult -> CheckResultSteamID
// plus the persona ISteamFriends holds for the same account.
//
//   node test/inventory-proof.js <appid> [defid,defid] [--grant]
//
// --grant first calls AddPromoItems on the listed defs (promo: "manual"), so a
// fresh account can be given the items it then proves. The proof's byte size
// is printed against lobby chat's 4096-byte cap.

const { init } = require('../index.js')

const appId = Number(process.argv[2] || 480)
const defs = (process.argv[3] || '').split(',').filter(Boolean).map(Number)
const grant = process.argv.includes('--grant')

const client = init(appId)
const self = client.localplayer.getSteamId().steamId64

;(async () => {
    const persona = client.friends.getPersona(self)
    console.log('persona', {
        name: persona.name,
        state: persona.state,
        avatarBytes: persona.mediumAvatar ? persona.mediumAvatar.length : null,
    })

    if (grant) {
        const granted = await client.inventory.addPromoItems(defs)
        console.log('granted', granted.items)
    }

    const all = await client.inventory.getAllItems()
    const held = all.items.filter(item => defs.length === 0 || defs.includes(item.definition))
    console.log(`holding ${held.length} matching item stack(s) of ${all.items.length}`)
    if (held.length === 0) process.exit(1)

    const proof = await client.inventory.serializeItemsById(held.map(item => item.itemId))
    console.log(`serialized ${proof.items.length} item(s) into ${proof.buffer.length} bytes (lobby chat cap 4096)`)

    const verified = await client.inventory.deserializeResult(proof.buffer, self)
    console.log('verified', { items: verified.items.length, expired: verified.expired, belongsTo: verified.belongsTo })

    const someoneElse = await client.inventory.deserializeResult(proof.buffer, self + 1n)
    console.log('checked against another id', { belongsTo: someoneElse.belongsTo })

    const forged = Buffer.from(proof.buffer)
    forged[forged.length - 1] ^= 0xff
    await client.inventory.deserializeResult(forged, self).then(
        result => console.log('a forged proof resolved (it should not)', result),
        err => console.log('a forged proof is refused:', err.message)
    )

    process.exit(0)
})().catch(err => {
    console.error(err)
    process.exit(1)
})

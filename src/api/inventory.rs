use napi_derive::napi;

/// The Steam Inventory Service (`ISteamInventory`).
///
/// Every request that talks to Steam returns a promise that resolves with the
/// items the finished result describes, or rejects with the name of the
/// `EResult` Steam reported (for instance `RateLimitExceeded`), so callers can
/// branch on it.
#[napi]
pub mod inventory {
    use napi::bindgen_prelude::{BigInt, Buffer, Error};
    use std::collections::HashMap;
    use steamworks::{
        InventoryResult as SteamInventoryResult, ItemDefId, ItemInstanceId, SteamError, SteamId,
    };
    use tokio::sync::oneshot;

    #[napi(object)]
    pub struct InventoryItem {
        /// The instance id of the item stack.
        pub item_id: BigInt,
        /// The item definition id.
        pub definition: i32,
        /// `SteamItemDetails_t.m_unQuantity`, a uint16. Steam does not cap a stack
        /// at 65,535, so past that the read wraps mod 65,536 (90,001 reads 24,465),
        /// through `getAllItems`, a write's result and a serialized result alike.
        /// The `quantity` property from `getItemProperties` is the true figure.
        pub quantity: u32,
        /// `ESteamItemFlags` bits: 1 no-trade, 256 removed, 512 consumed.
        pub flags: u32,
    }

    #[napi(object)]
    pub struct InventoryResult {
        /// Server time the result was generated, unix seconds.
        pub timestamp: u32,
        pub items: Vec<InventoryItem>,
    }

    /// A result signed by Steam, for another player to verify.
    #[napi(object)]
    pub struct SerializedResult {
        /// Server time the result was generated, unix seconds.
        pub timestamp: u32,
        pub items: Vec<InventoryItem>,
        /// The signed bytes to hand to another player's `deserializeResult`.
        pub buffer: Buffer,
    }

    /// Another player's serialized result, verified by Steam.
    #[napi(object)]
    pub struct VerifiedResult {
        /// Server time the result was generated, unix seconds.
        pub timestamp: u32,
        pub items: Vec<InventoryItem>,
        /// The signature verified, but the set is over an hour old
        /// (`k_EResultExpired`): an item in it may since have been traded or consumed.
        pub expired: bool,
        /// `CheckResultSteamID` against the id the caller named. False means
        /// the sender handed over somebody else's inventory.
        pub belongs_to: bool,
        /// Every item of the verified set with its `GetResultItemProperty`
        /// properties, as `getItemProperties` reads them: the `quantity`
        /// property is the stack's true size past the uint16 read in `items`.
        pub properties: Vec<ItemProperties>,
    }

    #[napi(object)]
    pub struct DefinitionQuantity {
        pub definition: i32,
        pub quantity: u32,
    }

    #[napi(object)]
    pub struct InstanceQuantity {
        pub item_id: BigInt,
        pub quantity: u32,
    }

    /// One item of a result with every dynamic property Steam reports for it.
    #[napi(object)]
    pub struct ItemProperties {
        /// The instance id of the item stack.
        pub item_id: BigInt,
        /// The item definition id.
        pub definition: i32,
        /// `SteamItemDetails_t.m_unQuantity`, a uint16: a stack past 65,535 reads wrapped.
        pub quantity: u32,
        /// Every property `GetResultItemProperty` names for the item (a null
        /// name lists them), each with its value.
        pub properties: HashMap<String, String>,
    }

    fn instance(id: &BigInt) -> ItemInstanceId {
        ItemInstanceId(id.get_u64().1)
    }

    fn items_of(result: &SteamInventoryResult) -> Vec<InventoryItem> {
        result
            .items()
            .into_iter()
            .map(|item| InventoryItem {
                item_id: BigInt::from(item.item_id.0),
                definition: item.definition.0,
                quantity: item.quantity as u32,
                flags: item.flags.0 as u32,
            })
            .collect()
    }

    fn reason(e: SteamError) -> Error {
        Error::from_reason(format!("{:?}", e))
    }

    /// Issues one request and waits for the finished result, handle and all.
    async fn raw(
        submit: impl FnOnce(
            steamworks::Inventory,
            Box<dyn FnOnce(Result<SteamInventoryResult, SteamError>) + Send>,
        ),
    ) -> Result<SteamInventoryResult, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        submit(
            client.inventory(),
            Box::new(move |result| {
                let _ = tx.send(result);
            }),
        );
        rx.await
            .map_err(|_| Error::from_reason("inventory request dropped"))?
            .map_err(reason)
    }

    async fn request(
        submit: impl FnOnce(
            steamworks::Inventory,
            Box<dyn FnOnce(Result<SteamInventoryResult, SteamError>) + Send>,
        ),
    ) -> Result<InventoryResult, Error> {
        let result = raw(submit).await?;
        Ok(InventoryResult {
            timestamp: result.timestamp(),
            items: items_of(&result),
        })
    }

    fn properties_of(result: &SteamInventoryResult) -> Vec<ItemProperties> {
        result
            .items()
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let index = index as u32;
                let properties = result
                    .item_property(index, None)
                    .unwrap_or_default()
                    .split(',')
                    .map(str::trim)
                    .filter(|name| !name.is_empty())
                    .map(|name| {
                        let value = result.item_property(index, Some(name)).unwrap_or_default();
                        (name.to_string(), value)
                    })
                    .collect();
                ItemProperties {
                    item_id: BigInt::from(item.item_id.0),
                    definition: item.definition.0,
                    quantity: item.quantity as u32,
                    properties,
                }
            })
            .collect()
    }

    /// Answered from the client's cache: a write's new instances appear seconds
    /// later, while the write's own result is current.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetAllItems}
    #[napi]
    pub async fn get_all_items() -> Result<InventoryResult, Error> {
        request(|inv, cb| inv.get_all_items(cb)).await
    }

    /// Lags a write similarly (about 15 s for a freshly made item).
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemsByID}
    #[napi]
    pub async fn get_items_by_id(item_ids: Vec<BigInt>) -> Result<InventoryResult, Error> {
        let ids: Vec<ItemInstanceId> = item_ids.iter().map(instance).collect();
        request(move |inv, cb| inv.get_items_by_id(&ids, cb)).await
    }

    /// `GetItemsByID`, then `GetResultItemProperty` on each item: first with a
    /// null name for the comma-separated property names, then each name's value.
    /// The `quantity` property is the true stack size. Right after a write it can
    /// be missing for the write's own rows.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetResultItemProperty}
    #[napi]
    pub async fn get_item_properties(item_ids: Vec<BigInt>) -> Result<Vec<ItemProperties>, Error> {
        let ids: Vec<ItemInstanceId> = item_ids.iter().map(instance).collect();
        let result = raw(move |inv, cb| inv.get_items_by_id(&ids, cb)).await?;
        Ok(properties_of(&result))
    }

    /// `GetAllItems`, then every item's properties as `getItemProperties` reads them.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetResultItemProperty}
    #[napi]
    pub async fn get_all_item_properties() -> Result<Vec<ItemProperties>, Error> {
        let result = raw(|inv, cb| inv.get_all_items(cb)).await?;
        Ok(properties_of(&result))
    }

    /// `GetItemsByID` on the local user's items, then `SerializeResult`: a
    /// minimal signed set another player can verify with `deserializeResult`.
    /// It expires an hour after `timestamp`.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
    #[napi]
    pub async fn serialize_items_by_id(item_ids: Vec<BigInt>) -> Result<SerializedResult, Error> {
        let ids: Vec<ItemInstanceId> = item_ids.iter().map(instance).collect();
        let result = raw(move |inv, cb| inv.get_items_by_id(&ids, cb)).await?;
        let buffer = result
            .serialize()
            .ok_or_else(|| Error::from_reason("SerializeResult failed"))?;
        Ok(SerializedResult {
            timestamp: result.timestamp(),
            items: items_of(&result),
            buffer: buffer.into(),
        })
    }

    /// `DeserializeResult` on bytes another player serialized, then
    /// `CheckResultSteamID` against the player who sent them, and each item's
    /// `GetResultItemProperty` properties. Rejects with the
    /// `EResult` name when the signature does not verify; an expired set
    /// resolves with `expired: true`.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#DeserializeResult}
    #[napi]
    pub async fn deserialize_result(
        buffer: Buffer,
        steam_id64: BigInt,
    ) -> Result<VerifiedResult, Error> {
        let bytes: Vec<u8> = buffer.to_vec();
        let sender = SteamId::from_raw(steam_id64.get_u64().1);
        let result = raw(move |inv, cb| inv.deserialize_result(&bytes, cb)).await?;
        Ok(VerifiedResult {
            timestamp: result.timestamp(),
            items: items_of(&result),
            expired: matches!(result.status(), Err(SteamError::Expired)),
            belongs_to: result.check_steam_id(sender),
            properties: properties_of(&result),
        })
    }

    /// `quantity` is a u32 and acts on the real quantity.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#ConsumeItem}
    #[napi]
    pub async fn consume_item(item_id: BigInt, quantity: u32) -> Result<InventoryResult, Error> {
        let id = instance(&item_id);
        request(move |inv, cb| inv.consume_item(id, quantity, cb)).await
    }

    /// `generate` must be exactly one item at quantity 1. An exchange's bundle is
    /// not capped. Writes are queued, not rejected: about 7 per second per client,
    /// shared with `generateItems`; past that latency rises and Steam never
    /// answers with a rate-limit error.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#ExchangeItems}
    #[napi]
    pub async fn exchange_items(
        generate: Vec<DefinitionQuantity>,
        destroy: Vec<InstanceQuantity>,
    ) -> Result<InventoryResult, Error> {
        let generate: Vec<(ItemDefId, u32)> = generate
            .iter()
            .map(|g| (ItemDefId(g.definition), g.quantity))
            .collect();
        let destroy: Vec<(ItemInstanceId, u32)> = destroy
            .iter()
            .map(|d| (instance(&d.item_id), d.quantity))
            .collect();
        request(move |inv, cb| inv.exchange_items(&generate, &destroy, cb)).await
    }

    fn signed(result: SteamInventoryResult) -> Result<SerializedResult, Error> {
        let buffer = result
            .serialize()
            .ok_or_else(|| Error::from_reason("SerializeResult failed"))?;
        Ok(SerializedResult {
            timestamp: result.timestamp(),
            items: items_of(&result),
            buffer: buffer.into(),
        })
    }

    /// `ConsumeItem`, then `SerializeResult` on that request's own result: a
    /// proof of the spend another player can verify with `deserializeResult`,
    /// with no `GetItemsByID` read between the write and the signature. The
    /// emptied row is signed at quantity 0 with the consumed flag; a proof still
    /// verifies after its item is consumed.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
    #[napi]
    pub async fn consume_item_serialized(
        item_id: BigInt,
        quantity: u32,
    ) -> Result<SerializedResult, Error> {
        let id = instance(&item_id);
        signed(raw(move |inv, cb| inv.consume_item(id, quantity, cb)).await?)
    }

    /// `ExchangeItems`, then `SerializeResult` on that request's own result:
    /// what the exchange spent and made, signed, as `consumeItemSerialized`.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
    #[napi]
    pub async fn exchange_items_serialized(
        generate: Vec<DefinitionQuantity>,
        destroy: Vec<InstanceQuantity>,
    ) -> Result<SerializedResult, Error> {
        let generate: Vec<(ItemDefId, u32)> = generate
            .iter()
            .map(|g| (ItemDefId(g.definition), g.quantity))
            .collect();
        let destroy: Vec<(ItemInstanceId, u32)> = destroy
            .iter()
            .map(|d| (instance(&d.item_id), d.quantity))
            .collect();
        signed(raw(move |inv, cb| inv.exchange_items(&generate, &destroy, cb)).await?)
    }

    /// Developer-only. One call is capped somewhere between 100 and 1,000 units
    /// (1,000 fails with `k_EResultFail`). With `auto_stack`, every grant lands
    /// in the oldest stack (lowest instance id) of that definition.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GenerateItems}
    #[napi]
    pub async fn generate_items(items: Vec<DefinitionQuantity>) -> Result<InventoryResult, Error> {
        let items: Vec<(ItemDefId, u32)> = items
            .iter()
            .map(|g| (ItemDefId(g.definition), g.quantity))
            .collect();
        request(move |inv, cb| inv.generate_items(&items, cb)).await
    }

    /// `quantity` is a u32 and acts on the real quantity. With no `dest` the
    /// units split into a new instance that stays separate. Asking for more than
    /// the stack holds fails with `k_EResultFail` and moves nothing.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#TransferItemQuantity}
    #[napi]
    pub async fn transfer_item_quantity(
        source: BigInt,
        quantity: u32,
        dest: Option<BigInt>,
    ) -> Result<InventoryResult, Error> {
        let source = instance(&source);
        let dest = dest.as_ref().map(instance);
        request(move |inv, cb| inv.transfer_item_quantity(source, quantity, dest, cb)).await
    }

    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#TriggerItemDrop}
    #[napi]
    pub async fn trigger_item_drop(definition: i32) -> Result<InventoryResult, Error> {
        request(move |inv, cb| inv.trigger_item_drop(ItemDefId(definition), cb)).await
    }

    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GrantPromoItems}
    #[napi]
    pub async fn grant_promo_items() -> Result<InventoryResult, Error> {
        request(|inv, cb| inv.grant_promo_items(cb)).await
    }

    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#AddPromoItems}
    #[napi]
    pub async fn add_promo_items(definitions: Vec<i32>) -> Result<InventoryResult, Error> {
        let defs: Vec<ItemDefId> = definitions.into_iter().map(ItemDefId).collect();
        request(move |inv, cb| inv.add_promo_items(&defs, cb)).await
    }

    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#SendItemDropHeartbeat}
    #[napi]
    pub fn send_item_drop_heartbeat() {
        crate::client::get_client()
            .inventory()
            .send_item_drop_heartbeat()
    }

    /// Starts loading the item definitions; the `SteamInventoryDefinitionUpdate`
    /// callback fires when they are available.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#LoadItemDefinitions}
    #[napi]
    pub fn load_item_definitions() -> bool {
        crate::client::get_client()
            .inventory()
            .load_item_definitions()
    }

    /// The ids of every loaded item definition, or `null` before they load.
    /// Lists only `item` definitions, never bundles or generators (their
    /// properties read null).
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemDefinitionIDs}
    #[napi]
    pub fn get_item_definition_ids() -> Option<Vec<i32>> {
        crate::client::get_client()
            .inventory()
            .item_definition_ids()
            .map(|ids| ids.into_iter().map(|id| id.0).collect())
    }

    /// A property of a loaded definition; with no name, the comma-separated
    /// property names.
    /// {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemDefinitionProperty}
    #[napi]
    pub fn get_item_definition_property(definition: i32, name: Option<String>) -> Option<String> {
        crate::client::get_client()
            .inventory()
            .item_definition_property(ItemDefId(definition), name.as_deref())
    }
}

export declare function init(appId?: number | undefined | null): void
export declare function restartAppIfNecessary(appId: number): boolean
export declare function runCallbacks(): void
export interface PlayerSteamId {
  steamId64: bigint
  steamId32: string
  accountId: number
}
export declare namespace achievement {
  export function activate(achievement: string): boolean
  export function isActivated(achievement: string): boolean
  export function clear(achievement: string): boolean
  export function names(): Array<string>
}
export declare namespace apps {
  export function isSubscribedApp(appId: number): boolean
  export function isAppInstalled(appId: number): boolean
  export function isDlcInstalled(appId: number): boolean
  export function isSubscribedFromFreeWeekend(): boolean
  export function isVacBanned(): boolean
  export function isCybercafe(): boolean
  export function isLowViolence(): boolean
  export function isSubscribed(): boolean
  export function appBuildId(): number
  export function appInstallDir(appId: number): string
  export function appOwner(): PlayerSteamId
  export function availableGameLanguages(): Array<string>
  export function currentGameLanguage(): string
  export function currentBetaName(): string | null
}
export declare namespace auth {
  /**
   * @param steamId64 - The user steam id or game server steam id. Use as NetworkIdentity of the remote system that will authenticate the ticket. If it is peer-to-peer then the user steam ID. If it is a game server, then the game server steam ID may be used if it was obtained from a trusted 3rd party
   * @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
   */
  export function getSessionTicketWithSteamId(steamId64: bigint, timeoutSeconds?: number | undefined | null): Promise<Ticket>
  /**
   * @param ip - The string of IPv4 or IPv6 address. Use as NetworkIdentity of the remote system that will authenticate the ticket.
   * @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
   */
  export function getSessionTicketWithIp(ip: string, timeoutSeconds?: number | undefined | null): Promise<Ticket>
  export function getAuthTicketForWebApi(identity: string, timeoutSeconds?: number | undefined | null): Promise<Ticket>
  export class Ticket {
    cancel(): void
    getBytes(): Buffer
  }
}
export declare namespace callback {
  export const enum SteamCallback {
    PersonaStateChange = 0,
    SteamServersConnected = 1,
    SteamServersDisconnected = 2,
    SteamServerConnectFailure = 3,
    LobbyDataUpdate = 4,
    LobbyChatUpdate = 5,
    P2PSessionRequest = 6,
    P2PSessionConnectFail = 7,
    GameLobbyJoinRequested = 8,
    MicroTxnAuthorizationResponse = 9,
    SteamInventoryDefinitionUpdate = 10,
    SteamInventoryFullUpdate = 11
  }
  export function register<C extends keyof import('./callbacks').CallbackReturns>(steamCallback: C, handler: (value: import('./callbacks').CallbackReturns[C]) => void): Handle
  export class Handle {
    disconnect(): void
  }
}
export declare namespace cloud {
  export function isEnabledForAccount(): boolean
  export function isEnabledForApp(): boolean
  export function setEnabledForApp(enabled: boolean): void
  export function readFile(name: string): string
  export function writeFile(name: string, content: string): boolean
  export function deleteFile(name: string): boolean
  export function fileExists(name: string): boolean
  export function listFiles(): Array<FileInfo>
  export class FileInfo {
    name: string
    size: bigint
  }
}
export declare namespace input {
  export const enum InputType {
    Unknown = 'Unknown',
    SteamController = 'SteamController',
    XBox360Controller = 'XBox360Controller',
    XBoxOneController = 'XBoxOneController',
    GenericGamepad = 'GenericGamepad',
    PS4Controller = 'PS4Controller',
    AppleMFiController = 'AppleMFiController',
    AndroidController = 'AndroidController',
    SwitchJoyConPair = 'SwitchJoyConPair',
    SwitchJoyConSingle = 'SwitchJoyConSingle',
    SwitchProController = 'SwitchProController',
    MobileTouch = 'MobileTouch',
    PS3Controller = 'PS3Controller',
    PS5Controller = 'PS5Controller',
    SteamDeckController = 'SteamDeckController'
  }
  export interface AnalogActionVector {
    x: number
    y: number
  }
  export function init(): void
  export function getControllers(): Array<Controller>
  export function getActionSet(actionSetName: string): bigint
  export function getDigitalAction(actionName: string): bigint
  export function getAnalogAction(actionName: string): bigint
  export function shutdown(): void
  export class Controller {
    activateActionSet(actionSetHandle: bigint): void
    isDigitalActionPressed(actionHandle: bigint): boolean
    getAnalogActionVector(actionHandle: bigint): AnalogActionVector
    getType(): InputType
    getHandle(): bigint
  }
}
export declare namespace inventory {
  export interface InventoryItem {
    /** The instance id of the item stack. */
    itemId: bigint
    /** The item definition id. */
    definition: number
    /**
     * `SteamItemDetails_t.m_unQuantity`, a uint16. Steam does not cap a stack
     * at 65,535, so past that the read wraps mod 65,536 (90,001 reads 24,465),
     * through `getAllItems`, a write's result and a serialized result alike.
     * The `quantity` property from `getItemProperties` is the true figure.
     */
    quantity: number
    /** `ESteamItemFlags` bits: 1 no-trade, 256 removed, 512 consumed. */
    flags: number
  }
  export interface InventoryResult {
    /** Server time the result was generated, unix seconds. */
    timestamp: number
    items: Array<InventoryItem>
  }
  /** A result signed by Steam, for another player to verify. */
  export interface SerializedResult {
    /** Server time the result was generated, unix seconds. */
    timestamp: number
    items: Array<InventoryItem>
    /** The signed bytes to hand to another player's `deserializeResult`. */
    buffer: Buffer
  }
  /** Another player's serialized result, verified by Steam. */
  export interface VerifiedResult {
    /** Server time the result was generated, unix seconds. */
    timestamp: number
    items: Array<InventoryItem>
    /**
     * The signature verified, but the set is over an hour old
     * (`k_EResultExpired`): an item in it may since have been traded or consumed.
     */
    expired: boolean
    /**
     * `CheckResultSteamID` against the id the caller named. False means
     * the sender handed over somebody else's inventory.
     */
    belongsTo: boolean
    /**
     * Every item of the verified set with its `GetResultItemProperty`
     * properties, as `getItemProperties` reads them: the `quantity`
     * property is the stack's true size past the uint16 read in `items`.
     */
    properties: Array<ItemProperties>
  }
  export interface DefinitionQuantity {
    definition: number
    quantity: number
  }
  export interface InstanceQuantity {
    itemId: bigint
    quantity: number
  }
  /** One item of a result with every dynamic property Steam reports for it. */
  export interface ItemProperties {
    /** The instance id of the item stack. */
    itemId: bigint
    /** The item definition id. */
    definition: number
    /** `SteamItemDetails_t.m_unQuantity`, a uint16: a stack past 65,535 reads wrapped. */
    quantity: number
    /**
     * Every property `GetResultItemProperty` names for the item (a null
     * name lists them), each with its value.
     */
    properties: Record<string, string>
  }
  /**
   * Answered from the client's cache: a write's new instances appear seconds
   * later, while the write's own result is current.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetAllItems}
   */
  export function getAllItems(): Promise<InventoryResult>
  /**
   * Lags a write similarly (about 15 s for a freshly made item).
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemsByID}
   */
  export function getItemsById(itemIds: Array<bigint>): Promise<InventoryResult>
  /**
   * `GetItemsByID`, then `GetResultItemProperty` on each item: first with a
   * null name for the comma-separated property names, then each name's value.
   * The `quantity` property is the true stack size. Right after a write it can
   * be missing for the write's own rows.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetResultItemProperty}
   */
  export function getItemProperties(itemIds: Array<bigint>): Promise<Array<ItemProperties>>
  /**
   * `GetAllItems`, then every item's properties as `getItemProperties` reads them.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetResultItemProperty}
   */
  export function getAllItemProperties(): Promise<Array<ItemProperties>>
  /**
   * `GetItemsByID` on the local user's items, then `SerializeResult`: a
   * minimal signed set another player can verify with `deserializeResult`.
   * It expires an hour after `timestamp`.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
   */
  export function serializeItemsById(itemIds: Array<bigint>): Promise<SerializedResult>
  /**
   * `DeserializeResult` on bytes another player serialized, then
   * `CheckResultSteamID` against the player who sent them, and each item's
   * `GetResultItemProperty` properties. Rejects with the
   * `EResult` name when the signature does not verify; an expired set
   * resolves with `expired: true`.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#DeserializeResult}
   */
  export function deserializeResult(buffer: Buffer, steamId64: bigint): Promise<VerifiedResult>
  /**
   * `quantity` is a u32 and acts on the real quantity.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#ConsumeItem}
   */
  export function consumeItem(itemId: bigint, quantity: number): Promise<InventoryResult>
  /**
   * `generate` must be exactly one item at quantity 1. An exchange's bundle is
   * not capped. Writes are queued, not rejected: about 7 per second per client,
   * shared with `generateItems`; past that latency rises and Steam never
   * answers with a rate-limit error.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#ExchangeItems}
   */
  export function exchangeItems(generate: Array<DefinitionQuantity>, destroy: Array<InstanceQuantity>): Promise<InventoryResult>
  /**
   * `ConsumeItem`, then `SerializeResult` on that request's own result: a
   * proof of the spend another player can verify with `deserializeResult`,
   * with no `GetItemsByID` read between the write and the signature. The
   * emptied row is signed at quantity 0 with the consumed flag; a proof still
   * verifies after its item is consumed.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
   */
  export function consumeItemSerialized(itemId: bigint, quantity: number): Promise<SerializedResult>
  /**
   * `ExchangeItems`, then `SerializeResult` on that request's own result:
   * what the exchange spent and made, signed, as `consumeItemSerialized`.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#SerializeResult}
   */
  export function exchangeItemsSerialized(generate: Array<DefinitionQuantity>, destroy: Array<InstanceQuantity>): Promise<SerializedResult>
  /**
   * Developer-only. One call is capped somewhere between 100 and 1,000 units
   * (1,000 fails with `k_EResultFail`). With `auto_stack`, every grant lands
   * in the oldest stack (lowest instance id) of that definition.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GenerateItems}
   */
  export function generateItems(items: Array<DefinitionQuantity>): Promise<InventoryResult>
  /**
   * `quantity` is a u32 and acts on the real quantity. With no `dest` the
   * units split into a new instance that stays separate. Asking for more than
   * the stack holds fails with `k_EResultFail` and moves nothing.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#TransferItemQuantity}
   */
  export function transferItemQuantity(source: bigint, quantity: number, dest?: bigint | undefined | null): Promise<InventoryResult>
  /** {@link https://partner.steamgames.com/doc/api/ISteamInventory#TriggerItemDrop} */
  export function triggerItemDrop(definition: number): Promise<InventoryResult>
  /** {@link https://partner.steamgames.com/doc/api/ISteamInventory#GrantPromoItems} */
  export function grantPromoItems(): Promise<InventoryResult>
  /** {@link https://partner.steamgames.com/doc/api/ISteamInventory#AddPromoItems} */
  export function addPromoItems(definitions: Array<number>): Promise<InventoryResult>
  /** {@link https://partner.steamgames.com/doc/api/ISteamInventory#SendItemDropHeartbeat} */
  export function sendItemDropHeartbeat(): void
  /**
   * Starts loading the item definitions; the `SteamInventoryDefinitionUpdate`
   * callback fires when they are available.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#LoadItemDefinitions}
   */
  export function loadItemDefinitions(): boolean
  /**
   * The ids of every loaded item definition, or `null` before they load.
   * Lists only `item` definitions, never bundles or generators (their
   * properties read null).
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemDefinitionIDs}
   */
  export function getItemDefinitionIds(): Array<number> | null
  /**
   * A property of a loaded definition; with no name, the comma-separated
   * property names.
   * {@link https://partner.steamgames.com/doc/api/ISteamInventory#GetItemDefinitionProperty}
   */
  export function getItemDefinitionProperty(definition: number, name?: string | undefined | null): string | null
}
export declare namespace localplayer {
  export function getSteamId(): PlayerSteamId
  export function getName(): string
  export function getLevel(): number
  /** @returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in, e.g. "US" or "UK". */
  export function getIpCountry(): string
  export function setRichPresence(key: string, value?: string | undefined | null): void
}
export declare namespace matchmaking {
  export const enum LobbyType {
    Private = 0,
    FriendsOnly = 1,
    Public = 2,
    Invisible = 3
  }
  export function createLobby(lobbyType: LobbyType, maxMembers: number): Promise<Lobby>
  export function joinLobby(lobbyId: bigint): Promise<Lobby>
  export function getLobbies(): Promise<Array<Lobby>>
  export class Lobby {
    id: bigint
    join(): Promise<Lobby>
    leave(): void
    openInviteDialog(): void
    getMemberCount(): bigint
    getMemberLimit(): bigint | null
    getMembers(): Array<PlayerSteamId>
    getOwner(): PlayerSteamId
    setJoinable(joinable: boolean): boolean
    getData(key: string): string | null
    setData(key: string, value: string): boolean
    deleteData(key: string): boolean
    /** Get an object containing all the lobby data */
    getFullData(): Record<string, string>
    /**
     * Merge current lobby data with provided data in a single batch
     * @returns true if all data was set successfully
     */
    mergeFullData(data: Record<string, string>): boolean
  }
}
export declare namespace networking {
  export interface P2PPacket {
    data: Buffer
    size: number
    steamId: PlayerSteamId
  }
  /** The method used to send a packet */
  export const enum SendType {
    /**
     * Send the packet directly over udp.
     *
     * Can't be larger than 1200 bytes
     */
    Unreliable = 0,
    /**
     * Like `Unreliable` but doesn't buffer packets
     * sent before the connection has started.
     */
    UnreliableNoDelay = 1,
    /**
     * Reliable packet sending.
     *
     * Can't be larger than 1 megabyte.
     */
    Reliable = 2,
    /**
     * Like `Reliable` but applies the nagle
     * algorithm to packets being sent
     */
    ReliableWithBuffering = 3
  }
  export function sendP2PPacket(steamId64: bigint, sendType: SendType, data: Buffer): boolean
  export function isP2PPacketAvailable(): number
  export function readP2PPacket(size: number): P2PPacket
  export function acceptP2PSession(steamId64: bigint): void
}
export declare namespace overlay {
  export const enum Dialog {
    Friends = 0,
    Community = 1,
    Players = 2,
    Settings = 3,
    OfficialGameGroup = 4,
    Stats = 5,
    Achievements = 6
  }
  export const enum StoreFlag {
    None = 0,
    AddToCart = 1,
    AddToCartAndShow = 2
  }
  export function activateDialog(dialog: Dialog): void
  export function activateDialogToUser(dialog: Dialog, steamId64: bigint): void
  export function activateInviteDialog(lobbyId: bigint): void
  export function activateToWebPage(url: string): void
  export function activateToStore(appId: number, flag: StoreFlag): void
}
export declare namespace stats {
  export function getInt(name: string): number | null
  export function setInt(name: string, value: number): boolean
  export function store(): boolean
  export function resetAll(achievementsToo: boolean): boolean
}
export declare namespace utils {
  export function getAppId(): number
  export function getServerRealTime(): number
  export function isSteamRunningOnSteamDeck(): boolean
  export const enum GamepadTextInputMode {
    Normal = 0,
    Password = 1
  }
  export const enum GamepadTextInputLineMode {
    SingleLine = 0,
    MultipleLines = 1
  }
  /** @returns the entered text, or null if cancelled or could not show the input */
  export function showGamepadTextInput(inputMode: GamepadTextInputMode, inputLineMode: GamepadTextInputLineMode, description: string, maxCharacters: number, existingText?: string | undefined | null): Promise<string | null>
  export const enum FloatingGamepadTextInputMode {
    SingleLine = 0,
    MultipleLines = 1,
    Email = 2,
    Numeric = 3
  }
  /** @returns true if the floating keyboard was shown, otherwise, false */
  export function showFloatingGamepadTextInput(keyboardMode: FloatingGamepadTextInputMode, x: number, y: number, width: number, height: number): Promise<boolean>
}
export declare namespace workshop {
  export interface UgcResult {
    itemId: bigint
    needsToAcceptAgreement: boolean
  }
  export const enum UgcItemVisibility {
    Public = 0,
    FriendsOnly = 1,
    Private = 2,
    Unlisted = 3
  }
  export interface UgcUpdate {
    title?: string
    description?: string
    changeNote?: string
    previewPath?: string
    contentPath?: string
    tags?: Array<string>
    visibility?: UgcItemVisibility
  }
  export interface InstallInfo {
    folder: string
    sizeOnDisk: bigint
    timestamp: number
  }
  export interface DownloadInfo {
    current: bigint
    total: bigint
  }
  export const enum UpdateStatus {
    Invalid = 0,
    PreparingConfig = 1,
    PreparingContent = 2,
    UploadingContent = 3,
    UploadingPreviewFile = 4,
    CommittingChanges = 5
  }
  export interface UpdateProgress {
    status: UpdateStatus
    progress: bigint
    total: bigint
  }
  export function createItem(appId?: number | undefined | null): Promise<UgcResult>
  export function updateItem(itemId: bigint, updateDetails: UgcUpdate, appId?: number | undefined | null): Promise<UgcResult>
  export function updateItemWithCallback(itemId: bigint, updateDetails: UgcUpdate, appId: number | undefined | null, successCallback: (data: UgcResult) => void, errorCallback: (err: any) => void, progressCallback?: (data: UpdateProgress) => void, progressCallbackIntervalMs?: number | undefined | null): void
  /**
   * Subscribe to a workshop item. It will be downloaded and installed as soon as possible.
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#SubscribeItem}
   */
  export function subscribe(itemId: bigint): Promise<void>
  /**
   * Unsubscribe from a workshop item. This will result in the item being removed after the game quits.
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#UnsubscribeItem}
   */
  export function unsubscribe(itemId: bigint): Promise<void>
  /**
   * Gets the current state of a workshop item on this client. States can be combined.
   *
   * @returns a number with the current item state, e.g. 9
   * 9 = 1 (The current user is subscribed to this item) + 8 (The item needs an update)
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemState}
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#EItemState}
   */
  export function state(itemId: bigint): number
  /**
   * Gets info about currently installed content on the disc for workshop item.
   *
   * @returns an object with the the properties {folder, size_on_disk, timestamp}
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemInstallInfo}
   */
  export function installInfo(itemId: bigint): InstallInfo | null
  /**
   * Get info about a pending download of a workshop item.
   *
   * @returns an object with the properties {current, total}
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemDownloadInfo}
   */
  export function downloadInfo(itemId: bigint): DownloadInfo | null
  /**
   * Download or update a workshop item.
   *
   * @param highPriority - If high priority is true, start the download in high priority mode, pausing any existing in-progress Steam downloads and immediately begin downloading this workshop item.
   * @returns true or false
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#DownloadItem}
   */
  export function download(itemId: bigint, highPriority: boolean): boolean
  /**
   * Get all subscribed workshop items.
   * @returns an array of subscribed workshop item ids
   */
  export function getSubscribedItems(): Array<bigint>
  export function deleteItem(itemId: bigint): Promise<void>
  export const enum UGCQueryType {
    RankedByVote = 0,
    RankedByPublicationDate = 1,
    AcceptedForGameRankedByAcceptanceDate = 2,
    RankedByTrend = 3,
    FavoritedByFriendsRankedByPublicationDate = 4,
    CreatedByFriendsRankedByPublicationDate = 5,
    RankedByNumTimesReported = 6,
    CreatedByFollowedUsersRankedByPublicationDate = 7,
    NotYetRated = 8,
    RankedByTotalVotesAsc = 9,
    RankedByVotesUp = 10,
    RankedByTextSearch = 11,
    RankedByTotalUniqueSubscriptions = 12,
    RankedByPlaytimeTrend = 13,
    RankedByTotalPlaytime = 14,
    RankedByAveragePlaytimeTrend = 15,
    RankedByLifetimeAveragePlaytime = 16,
    RankedByPlaytimeSessionsTrend = 17,
    RankedByLifetimePlaytimeSessions = 18,
    RankedByLastUpdatedDate = 19
  }
  export const enum UGCType {
    Items = 0,
    ItemsMtx = 1,
    ItemsReadyToUse = 2,
    Collections = 3,
    Artwork = 4,
    Videos = 5,
    Screenshots = 6,
    AllGuides = 7,
    WebGuides = 8,
    IntegratedGuides = 9,
    UsableInGame = 10,
    ControllerBindings = 11,
    GameManagedItems = 12,
    All = 13
  }
  export const enum UserListType {
    Published = 0,
    VotedOn = 1,
    VotedUp = 2,
    VotedDown = 3,
    Favorited = 4,
    Subscribed = 5,
    UsedOrPlayed = 6,
    Followed = 7
  }
  export const enum UserListOrder {
    CreationOrderAsc = 0,
    CreationOrderDesc = 1,
    TitleAsc = 2,
    LastUpdatedDesc = 3,
    SubscriptionDateDesc = 4,
    VoteScoreDesc = 5,
    ForModeration = 6
  }
  export interface WorkshopItemStatistic {
    numSubscriptions?: bigint
    numFavorites?: bigint
    numFollowers?: bigint
    numUniqueSubscriptions?: bigint
    numUniqueFavorites?: bigint
    numUniqueFollowers?: bigint
    numUniqueWebsiteViews?: bigint
    reportScore?: bigint
    numSecondsPlayed?: bigint
    numPlaytimeSessions?: bigint
    numComments?: bigint
    numSecondsPlayedDuringTimePeriod?: bigint
    numPlaytimeSessionsDuringTimePeriod?: bigint
  }
  export interface WorkshopItem {
    publishedFileId: bigint
    creatorAppId?: number
    consumerAppId?: number
    title: string
    description: string
    owner: PlayerSteamId
    /** Time created in unix epoch seconds format */
    timeCreated: number
    /** Time updated in unix epoch seconds format */
    timeUpdated: number
    /** Time when the user added the published item to their list (not always applicable), provided in Unix epoch format (time since Jan 1st, 1970). */
    timeAddedToUserList: number
    visibility: UgcItemVisibility
    banned: boolean
    acceptedForUse: boolean
    tags: Array<string>
    tagsTruncated: boolean
    url: string
    numUpvotes: number
    numDownvotes: number
    numChildren: number
    previewUrl?: string
    statistics: WorkshopItemStatistic
  }
  export interface WorkshopPaginatedResult {
    items: Array<WorkshopItem | undefined | null>
    returnedResults: number
    totalResults: number
    wasCached: boolean
  }
  export interface WorkshopItemsResult {
    items: Array<WorkshopItem | undefined | null>
    wasCached: boolean
  }
  export interface WorkshopItemQueryConfig {
    cachedResponseMaxAge?: number
    includeMetadata?: boolean
    includeLongDescription?: boolean
    includeAdditionalPreviews?: boolean
    onlyIds?: boolean
    onlyTotal?: boolean
    language?: string
    matchAnyTag?: boolean
    requiredTags?: Array<string>
    excludedTags?: Array<string>
    searchText?: string
    rankedByTrendDays?: number
  }
  export interface AppIDs {
    creator?: number
    consumer?: number
  }
  export function getItem(item: bigint, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopItem | null>
  export function getItems(items: Array<bigint>, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopItemsResult>
  export function getAllItems(page: number, queryType: UGCQueryType, itemType: UGCType, creatorAppId: number, consumerAppId: number, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopPaginatedResult>
  export function getUserItems(page: number, accountId: number, listType: UserListType, itemType: UGCType, sortOrder: UserListOrder, appIds: AppIDs, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopPaginatedResult>
}

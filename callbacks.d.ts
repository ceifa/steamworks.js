import client = require('./client')

export interface CallbackReturns {
    [client.callback.SteamCallback.PersonaStateChange]: {
        steam_id: bigint
        flags: { bits: number }
    }
    [client.callback.SteamCallback.SteamServersConnected]: {}
    [client.callback.SteamCallback.SteamServersDisconnected]: {
        reason: number
    }
    [client.callback.SteamCallback.SteamServerConnectFailure]: {
        reason: number
        still_retrying: boolean
    }
}

import client = require('./client')

export const enum ChatMemberStateChange {
    /** This user has joined or is joining the lobby. */
    Entered,
    /** This user has left or is leaving the lobby. */
    Left,
    /** User disconnected without leaving the lobby first. */
    Disconnected,
    /** The user has been kicked. */
    Kicked,
    /** The user has been kicked and banned. */
    Banned,
}

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
    [client.callback.SteamCallback.LobbyDataUpdate]: {
        lobby: bigint
        member: bigint
        success: boolean
    }
    [client.callback.SteamCallback.LobbyChatUpdate]: {
        lobby: bigint
        user_changed: bigint
        making_change: bigint
        member_state_change: ChatMemberStateChange
    }
    [client.callback.SteamCallback.P2PSessionRequest]: {
        remote: bigint
    }
    [client.callback.SteamCallback.P2PSessionConnectFail]: {
        remote: bigint
        error: number
    }
}

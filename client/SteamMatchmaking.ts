import { Api, Verb } from './Api.js'

export class SteamMatchmaking {

    public static async CreateLobbyAsync(maxMembers: number): Promise<any> {
        return await Api.send(`/matchmaking/createLobbyAsync/${maxMembers}`, Verb.Get)
    }

}

import { Api, Verb } from './Api.js'

export class SteamFriends {

    public static async GetFriends(): Promise<any> {
        return await Api.send(`/friends/friends`, Verb.Get)
    }

}

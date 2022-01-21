import { Api, Verb } from './Api.js'

export namespace SteamData {

    export class Lobby {

        public static async InviteFriend(friendId: number): Promise<any> {
            return await Api.send(`/data/inviteFriend/${friendId}`, Verb.Get)
        }

    }
}

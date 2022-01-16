import { Api, Verb } from './Api.js'

export class SteamAchievement {
    public static async activate(name: string): Promise<void> {
        await Api.send(`/achievement/${name}/activate`, Verb.Post)
    }
}

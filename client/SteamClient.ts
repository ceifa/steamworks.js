import { Api, Verb } from './Api.js'

export class SteamClient {
    public static async init(appId: number): Promise<void> {
        await Api.initialize()
        await Api.send(`/client/init/${appId}`, Verb.POST)
    }

    public static async getName(): Promise<string> {
        return await Api.send(`/client/name`, Verb.GET)
    }
}

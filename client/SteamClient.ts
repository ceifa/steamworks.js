import { Api, Verb } from './Api.js'

export class SteamClient {
    public static async init(appId: number): Promise<void> {
        await Api.initialize()
        await Api.send(`/client/init/${appId}`, Verb.Post)
    }

    public static async getName(): Promise<string> {
        return await Api.send(`/client/name`, Verb.Get)
    }

    public static async getSteamId(): Promise<{
        steamId64: string
        accountId: number
        isValid: boolean
    }> {
        return await Api.send(`/client/steamid`, Verb.Get)
    }

    public static async getSteamLevel(): Promise<number> {
        return await Api.send(`/client/level`, Verb.Get)
    }

    public static async getIpCountry(): Promise<string> {
        return await Api.send(`/client/ipcontry`, Verb.Get)
    }
}

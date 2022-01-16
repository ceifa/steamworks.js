import { Api, Verb } from './Api.js'

export class SteamStats {
    public static async getFloat(name: string): Promise<number> {
        return await Api.send(`/stats/${name}/float`, Verb.Get)
    }

    public static async getInt(name: string): Promise<number> {
        return await Api.send(`/stats/${name}/int`, Verb.Get)
    }

    public static async setFloat(name: string, value: number): Promise<void> {
        await Api.send(`/stats/${name}/float`, Verb.Post, value)
    }

    public static async setInt(name: string, value: number): Promise<void> {
        await Api.send(`/stats/${name}/int`, Verb.Post, value)
    }
}

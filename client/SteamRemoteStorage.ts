import { Api, Verb } from './Api.js'

export class SteamRemoteStorage {
    public static async isCloudEnabled(): Promise<boolean> {
        return await Api.send('/remotestorage/enabled', Verb.Get)
    }

    public static async deleteFile(name: string): Promise<boolean> {
        return await Api.send(`/remotestorage/${name}`, Verb.Delete)
    }

    public static async hasFile(name: string): Promise<boolean> {
        return await Api.send(`/remotestorage/${name}/exists`, Verb.Get)
    }

    public static async readFile(name: string): Promise<boolean> {
        return await Api.send(`/remotestorage/${name}`, Verb.Get)
    }

    public static async writeFile(name: string, content: string): Promise<boolean> {
        return await Api.send(`/remotestorage/${name}`, Verb.Post, content)
    }
}

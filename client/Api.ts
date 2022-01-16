import { connect, Socket } from 'node:net'
import { spawn } from 'node:child_process'
import * as path from 'node:path'

export enum Verb {
    GET,
    POST,
}

export type Message = {
    correlationId: string
    route: string
    verb: Verb
    content?: any
}

export class Api {
    private static waitingResolvers: Record<string, (value: any) => void> = {}
    private static client: Socket

    public static async initialize(): Promise<void> {
        const initializeClient = new Promise<void>((resolve) => {
            const steamworks = spawn(path.resolve('./dist/steamworks.js.exe'))

            steamworks.stdout.on('data', (data) => {
                data = data.toString().trim()

                if (data === 'Steamworks.js server started.') {
                    resolve()
                }

                console.log(`Steamworks: ${data}`)
            })

            steamworks.stderr.on('data', (data) => {
                console.error(`Steamworks: ${data.toString().trim()}`)
            })

            process.on('exit', () => steamworks.kill())
        })

        await initializeClient

        this.client = connect(45162, '127.0.0.1')
        this.client.on('data', (buffer) => {
            const message = JSON.parse(buffer.toString()) as Message
            this.waitingResolvers[message.correlationId](message.content)
        })

        console.log('Connected to steamworks server')
    }

    public static async send<T>(route: string, verb: Verb, content?: any): Promise<T> {
        if (!this.client) {
            throw new Error('Client not initialized')
        }

        const correlationId = Math.random().toString(36).substring(2, 15)
        const promise = new Promise<T>((resolver) => {
            this.waitingResolvers[correlationId] = resolver
        })

        this.client.write(
            JSON.stringify({
                correlationId,
                route,
                verb,
                content,
            }),
        )

        return await promise
    }
}

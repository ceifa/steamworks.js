export function init(appId?: number): Omit<Client, 'init' | 'runCallbacks'>;
export type Client = typeof import('./client.d');
export const SteamCallback: typeof import("./client.d").callback.SteamCallback;

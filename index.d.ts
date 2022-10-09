export function init(appId?: number): Omit<Client, 'init' | 'runCallbacks'>;
export function restartAppIfNecessary(appId: number): boolean;
export function electronEnableSteamOverlay(disableEachFrameInvalidation?: boolean): void;
export type Client = typeof import('./client.d');
export const SteamCallback: typeof import("./client.d").callback.SteamCallback;

import * as steamworks from "steamworks.js";

export default function main() {
	const client = steamworks.init(480);
	console.log(client.localplayer.getName())
}

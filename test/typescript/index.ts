import * as steamworks from "steamworks.js";

export default function main() {

	// Initialization without app ID should work!
	steamworks.init();

	// Initialization with numeric app ID should work!
	steamworks.init(480);
}

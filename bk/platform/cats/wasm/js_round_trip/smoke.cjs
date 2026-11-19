// TODO Reviewed ??-??-2026.

// ANCHOR: example
const assert = require("node:assert/strict");
const path = require("node:path");

const { greet } = require(path.resolve(process.argv[2]));
assert.equal(greet("  Zoë 🌱  "), "Hello, Zoë 🌱!");
assert.throws(() => greet("  "), {
  name: "Error",
  message: "Name must not be empty",
});
console.log("Wasm greeting and JavaScript error checked.");
// ANCHOR_END: example

import * as rustLib from 'rust-lib-wasm';

console.log("starting computation...");
const result = rustLib.concatenate_strings("a", "b");
console.log(`done computing, result: '${result}'`);
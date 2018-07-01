const fs = require("fs");
const path = require("path");
const child_process = require("child_process");

// TODO: Make this work on Unix
const compilerPath = path.resolve(__dirname, "../target/debug/bfcrs.exe");

child_process.execFileSync(compilerPath, [
  path.resolve(__dirname, "../bf/hello.bf")
]);

const buffer = fs.readFileSync(path.resolve(__dirname, "../bin/out.wasmb"));

WebAssembly.instantiate(buffer, {
  bfcrs: {
    print: x => process.stdout.write(String.fromCharCode(x)),
    read: () => 0
  }
}).then(mod => {
  mod.instance.exports.main();
});

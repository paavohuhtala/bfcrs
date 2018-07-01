const fs = require("fs");
const path = require("path");
const child_process = require("child_process");

const binaryPath = process.argv[2];
const buffer = fs.readFileSync(path.resolve(__dirname, "../", binaryPath));

WebAssembly.instantiate(buffer, {
  bfcrs: {
    print: x => process.stdout.write(String.fromCharCode(x)),
    read: () => 0
  }
}).then(mod => {
  mod.instance.exports.main();
});

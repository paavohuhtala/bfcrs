const fs = require("fs");

const waitReadable = stream => {
  return new Promise(resolve => {
    const listener = () => {
      resolve();
      stream.removeListener("readable", listener);
    };

    stream.on("readable", listener);
  });
};

const readMessage = async stream => {
  await waitReadable(process.stdin);
  const buffer = stream.read(4);
  const len = buffer.readUInt32LE(0);
  return stream.read(len);
};

const writeMessage = (stream, message) => {
  const buffer = Buffer.alloc(message.length + 4);
  buffer.writeUInt32LE(message.length, 0);
  message.copy(buffer, 4);

  return new Promise(resolve => {
    stream.write(buffer, () => resolve());
  });
};

const sendState = (stream, pointer, memory) => {
  const stateBuffer = Buffer.alloc(memory.length + 4);

  stateBuffer.writeUInt32LE(pointer, 0);
  memory.copy(stateBuffer, 4);

  return writeMessage(stream, stateBuffer);
};

(async function() {
  const buffer = await readMessage(process.stdin);

  let output = "";

  const mod = await WebAssembly.instantiate(buffer, {
    bfcrs: {
      print: x => {
        output += String.fromCharCode(x);
      },
      read: () => 0
    }
  });

  const pointer = mod.instance.exports.main();

  await writeMessage(process.stdout, Buffer.from(output));

  const memory = Buffer.from(
    new Uint8Array(mod.instance.exports.memory.buffer, 0)
  );

  await sendState(process.stdout, pointer, memory);

  await readMessage(process.stdin);

  process.exit(0);
})().catch(err => {
  fs.writeFileSync("./crash.log", err.stack);
  process.exit(1);
});

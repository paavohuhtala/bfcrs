document.addEventListener("DOMContentLoaded", () => {
  const outputElement = document.getElementById("output");

  document.getElementById("moduleInput").addEventListener("change", function() {
    const reader = new FileReader();
    reader.onload = function() {
      let output = "";
      WebAssembly.instantiate(this.result, {
        bfcrs: {
          print: x => {
            output += String.fromCharCode(x);
            outputElement.innerText = output;
            console.log(x);
          },
          read: () => 0
        }
      })
        .then(mod => {
          mod.instance.exports.main();
        })
        .catch(x => console.error(x));
    };

    reader.readAsArrayBuffer(this.files[0]);
  });
});

// https://github.com/torch2424/wasm-by-example/blob/master/demo-util/
export const wasmBrowserInstantiate = async (wasmModuleUrl, importObject) => {
    let response = undefined;

    // Check if the browser supports streaming instantiation
    if (WebAssembly.instantiateStreaming) {
    // Fetch the module, and instantiate it as it is downloading
    response = await WebAssembly.instantiateStreaming(
      fetch(wasmModuleUrl),
      importObject
    );
    } else {
    // Fallback to using fetch to download the entire module
    // And then instantiate the module
    const fetchAndInstantiateTask = async () => {
      const wasmArrayBuffer = await fetch(wasmModuleUrl).then(response =>
        response.arrayBuffer()
      );
      return WebAssembly.instantiate(wasmArrayBuffer, importObject);
    };

    response = await fetchAndInstantiateTask();
    }

    return response;
};

function toString(wasmModule, stringPosition) {
    const memory = wasmModule.instance.exports.memory;
    const extractedBuffer = new Uint8Array(memory.buffer, stringPosition, 30);
    return new TextDecoder("utf8").decode(extractedBuffer);
}

const go = new Go(); // Defined in wasm_exec.js. Don't forget to add this in your index.html.

const runWasmAdd = async () => {
    // Get the importObject from the go instance.
    const importObject = go.importObject;

    // Instantiate our wasm module
    const wasmModule = await wasmBrowserInstantiate("../saymyname/saymyname.wasm", importObject);

    // Allow the wasm_exec go instance, bootstrap and execute our wasm module
    go.run(wasmModule.instance);

    // Call the Add function export from wasm, save the result
    const addResult = wasmModule.instance.exports.add(24, 24);

    // Call the sayMyname function
    const helloStringPosition = wasmModule.instance.exports.sayMyName()
    const myName = toString(wasmModule, helloStringPosition)

    // Set the result onto the body
    //document.body.textContent = `Addresult: ${addResult}`;
    document.body.textContent = ` - ${myName}`;
};

runWasmAdd();
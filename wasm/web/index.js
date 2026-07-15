const go = new Go(); // Defined in wasm_exec.js

const runWasm = async () => {
    let wasmModule;

    if (WebAssembly.instantiateStreaming) {
        wasmModule = await WebAssembly.instantiateStreaming(
            fetch("../saymyname/saymyname.wasm"),
            go.importObject
        );
    } else {
        const buf = await fetch("./saymyname.wasm").then(r => r.arrayBuffer());
        wasmModule = await WebAssembly.instantiate(buf, go.importObject);
    }

    // Start the Go runtime (registers globals: add, sayMyName)
    go.run(wasmModule.instance);

    // Give the Go runtime one tick to register the JS globals
    await new Promise(resolve => setTimeout(resolve, 0));

    const addResult = window.add(24, 24);
    const myName = window.sayMyName("Slim Shady");

    document.body.textContent = `add(24,24) = ${addResult} | ${myName}`;
};

runWasm();

# ark-circom in wasm

> Warning: ⚠️ This is an extremely hacky project. 

I just quickly tried to make ark-circom work in wasm. The cool thing about it is that you can run the wasm generator inside wasm (🤯). However, the performance is pretty bad, so probably much better to run the wasm witness generator directly in the browser runtime and pass the witness as vec.

However, it's probably also useful for someone that just wants to get the prover work in the browser. Docs for wasm_bindgen_rayon were not amazing. I also had to serve the wasm and html, since it requires specific CORS headers to be set.

## Run

1. Build the wasm: `wasm-pack build --target web`

2. Serve the files: `python3 server.py`

3. Open `localhost:8000` and you should see some proof printed in the dev console 🎉

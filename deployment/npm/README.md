# @dprint/nix

npm distribution of [dprint-plugin-nix](https://github.com/dprint/dprint-plugin-nix) which is an adapter plugin for [nixfmt-rs](https://github.com/Mic92/nixfmt-rs).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-nix Wasm file](https://github.com/dprint/dprint-plugin-nix/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/nix";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText({
  filePath: "file.nix",
  fileText: "{\nfoo = 1;\n}\n",
}));
```

// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.nix",
  fileText: "{\nfoo = 1;\n}\n",
});

assert.strictEqual(result, "{\n  foo = 1;\n}\n");

# dprint-plugin-nix

[![npm version](https://img.shields.io/npm/v/@dprint/nix.svg)](https://www.npmjs.com/package/@dprint/nix) [![CI](https://github.com/dprint/dprint-plugin-nix/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-nix/actions?query=workflow%3ACI)

Adapter for [nixfmt-rs](https://github.com/Mic92/nixfmt-rs) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [Nix](https://nixos.org) files (`.nix`).

Originally created by [Kenichi Kamiya](https://github.com/kachick).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add nix
```

Note: You do not need nixfmt installed globally as dprint will run nixfmt-rs from the .wasm file in a sandboxed environment.

## Configuration

Works with no configuration by default. To add configuration, specify a `"nix"` key in your dprint.json:

```jsonc
{
  "nix": {
    "lineWidth": 100,
    "indentWidth": 2,
  },
  "plugins": [
    // ...etc...
  ],
}
```

## Tips

To format Nix code blocks in Markdown, use the `tags` option of the [Markdown plugin](https://github.com/dprint/dprint-plugin-markdown):

```json
{
  "markdown": {
    "tags": {
      "nix": "nix"
    }
  }
}
```

To run dprint with `nix fmt`, replace nixfmt-tree with dprint:

```nix
{
  formatter = forAllSystems (
    system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    pkgs.writeShellScriptBin "dprint-fmt" ''
      exec "${lib.getExe pkgs.dprint}" fmt "$@"
    ''
  );
}
```

## Limitation

Because this plugin is updated after upstream nixfmt and nixfmt-rs, it will lag behind the latest upstream.\
Please do not use it for nixpkgs or [NixOS/](https://github.com/NixOS/) contributions.

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/nix)

## Versioning

This repo automatically upgrades to the latest version of nixfmt-rs once a day. You can check which version of nixfmt-rs is being used by looking at the `nixfmt_rs` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-nix/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of nixfmt-rs.

## Acknowledgments

- Thanks to [nixfmt](https://github.com/NixOS/nixfmt) for the official Nix formatter.
- Thanks to [nixfmt-rs](https://github.com/Mic92/nixfmt-rs) for porting nixfmt to Rust, which makes it easy to use as a Wasm plugin.

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.

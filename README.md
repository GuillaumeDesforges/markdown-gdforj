# markdown-rs-cli

A light CLI tool to convert markdown files to HTML using [markdown-rs](https://github.com/wooorm/markdown-rs).

## Install

```console
$ nix shell github:GuillaumeDesforges/markdown-rs-cli
```

## Usage

Change your current working directory at the root of your project.

Then call the CLI:

```console
$ markdown-rs-cli
```

It will convert all markdown files in the current directory and its subdirectories to HTML,
writing them to a file in the directory `dist` with the same relative path.

For example, `README.md` will be converted to `dist/README.html`, and `docs/README.md` will be converted to `dist/docs/README.html`.

## Develop

Provide a Rust environment:

```console
$ nix develop
```

Note: if you want to use your own `nixpkgs` registered flake to avoid pulling another closure, you can run `nix develop -f devShell.nix` instead.

# markdown-gdforj

A light CLI tool to convert markdown files to HTML using [markdown-rs](https://github.com/wooorm/markdown-rs).

> [!CAUTION]
> This tool is my own flavor of using markdown, and is not meant to be used by anyone else.
> Use at your own risk.

## Install

```console
$ nix shell github:GuillaumeDesforges/markdown-gdforj
```

## Usage

Change your current working directory at the root of your project.

Then call the CLI:

```console
$ markdown-gdforj
```

It will convert all markdown files in the current directory and its subdirectories to HTML,
writing them to a file in the directory `dist` with the same relative path.

For example, `README.md` will be converted to `dist/README.html`, and `docs/README.md` will be converted to `dist/docs/README.html`.

It will also copy all image files that are referenced in the markdown files to the `dist` directory, with appropriate path.

## Develop

Provide a Rust environment:

```console
$ nix develop
```

Note: if you want to use your own `nixpkgs` registered flake to avoid pulling another closure, you can run `nix develop -f devShell.nix` instead.

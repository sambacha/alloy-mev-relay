# `[alloy-mev-relay-watch]`

> [!NOTE]
> Work in Progress 


## Adapting this template

- Run `nix develop` to have a working shell ready before name change.
- Change `name` in Cargo.toml.
- Run `cargo generate-lockfile` in the nix shell
- There are two CI workflows, and one of them uses Nix which is slower (unless you configure a cache) than the other that is based on rustup. Pick one or the other depending on your trade-offs.

## Development (Flakes)

This repo uses [Flakes](https://nixos.asia/en/flakes) from the get-go.

```bash
# Dev shell
nix develop

# or run via cargo
nix develop -c cargo run

# build
nix build
```

We also provide a [`justfile`](https://just.systems/) for Makefile'esque commands to be run inside of the devShell.

## See Also

- [nixos.wiki: Packaging Rust projects with nix](https://wiki.nixos.org/wiki/Rust#Packaging_Rust_projects_with_nix)

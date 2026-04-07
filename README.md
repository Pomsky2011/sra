# sra

**SRA** — Source Repositories for Applications. Rollbian's community package manager, similar to the AUR.

## Usage

```
sra search <query>       Search for packages
sra info <package>       Show package information
sra install <package>    Build and install a package
sra remove <package>     Remove an installed package
sra update               Update all installed SRA packages
sra submit <path>        Submit a new ROLLPKG to the index
```

## Building

```sh
cargo build --release
```

## License

LGPLv3 — see [LICENSE](LICENSE).

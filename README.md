# git-matrix

Git remote helper that lets you push and clone from matrix rooms.

## Usage

You'll need `cargo` to install git-matrix, [rustup](https://rustup.sh) if you don't have it yet.

```shell
cargo install --path=git_matrix_cli
git clone matrix://matrix.org/git-matrix
```

## Login

The remote helper acts as guest by default. To login execute

```shell
git matrix
```

which prompts for

```
Homeserver URL: https://example.org
User: @example:example.org
Password:
```

The resulting access token and device id are stored in the global git config.

## Custom Remote

```shell
git clone matrix::<scheme>://<domain>[:port]/<room-alias>
```

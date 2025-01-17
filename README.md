# Rover

> ✨ 🤖 🐶 the new CLI for apollo

[![Tests](https://github.com/apollographql/rover/workflows/Tests/badge.svg)](https://github.com/apollographql/rover/actions?query=workflow%3ATests)
![Stability: Experimental](https://img.shields.io/badge/stability-experimental-red)
[![Netlify Status](https://api.netlify.com/api/v1/badges/1646a37a-eb2b-48e8-b6c9-cd074f02bb50/deploy-status)](https://app.netlify.com/sites/apollo-cli-docs/deploys)

This is the home of Rover, the new CLI for Apollo's suite of GraphQL developer productivity tools.

### Note

This `README` contains just enough info to get you started with Rover. Our [docs](https://go.apollo.dev/r/docs) contain more detailed information that should be your primary reference for all things Rover.

## Usage

A few useful Rover commands to interact with your graphs:

1. Fetch a graph from a federated remote endpoint.

```bash
rover graph fetch test@cats
```

1. Validate recent changes made to your local graph with `rover graph check`.

```bash
rover graph check --schema=./path-to-valid-sdl test@cats
```

1. Publish your local graph to Apollo Studio.

```bash
rover graph publish --schema ./path-to-valid-schema test@cats
```

## Command-line options

```console
Rover 0.1.5

Rover - Your Graph Companion
Read the getting started guide by running:

    $ rover docs open start

To begin working with Rover and to authenticate with Apollo Studio,
run the following command:

    $ rover config auth

This will prompt you for an API Key that can be generated in Apollo Studio.

The most common commands from there are:

    - rover graph fetch: Fetch a graph schema from the Apollo graph registry
    - rover graph check: Check for breaking changes in a local graph schema against a graph schema in the Apollo graph
registry
    - rover graph publish: Publish an updated graph schema to the Apollo graph registry

You can open the full documentation for Rover by running:

    $ rover docs open

USAGE:
    rover [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --log <log-level>    Specify Rover's log level [possible values: error, warn, info,
                             debug, trace]

SUBCOMMANDS:
    config        Configuration profile commands
    docs          Interact with Rover's documentation
    explain       Explain error codes
    graph         Graph API schema commands
    help          Prints this message or the help of the given subcommand(s)
    subgraph      Subgraph schema commands
    supergraph    Supergraph schema commands
    update        Commands related to updating rover
```

This repo is organized as a [`cargo` workspace], containing several related projects:

- `rover`: Apollo's suite of GraphQL developer productivity tools
- [`houston`]: utilities for configuring Rover
- [`robot-panic`]: a fork of [`rust-cli/human-panic`] adjusted for Rover
- [`rover-client`]: an HTTP client for making GraphQL requests for Rover
- [`sdl-encoder`]: a crate to encode SDL
- [`sputnik`]: a crate to aid in collection of anonymous data for Rust CLIs
- [`timber`]: Rover's logging formatter

[`cargo` workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[`houston`]: https://github.com/apollographql/rover/tree/main/crates/houston
[`robot-panic`]: https://github.com/apollographql/rover/tree/main/crates/robot-panic
[`rust-cli/human-panic`]: https://github.com/rust-cli/human-panic
[`rover-client`]: https://github.com/apollographql/rover/tree/main/crates/rover-client
[`sdl-encoder`]: https://github.com/apollographql/rover/tree/main/crates/sdl-encoder
[`sputnik`]: https://github.com/apollographql/rover/tree/main/crates/sputnik
[`timber`]: https://github.com/apollographql/rover/tree/main/crates/timber

## Installation Methods

#### Linux and MacOS `curl | sh` installer

To install the latest release of Rover: 

```bash
curl -sSL https://rover.apollo.dev/nix/latest | sh
```

To install a specific version of Rover (note the `v` prefixing the version number):

> Note: If you're installing Rover in a CI environment, it's best to target a specific version rather than using the latest URL, since future major breaking changes could affect CI workflows otherwise.

```bash
curl -sSL https://rover.apollo.dev/nix/v0.1.1 | sh
```

You will need `curl` installed on your system to run the above installation commands. You can get the latest version from [the curl downloads page](https://curl.se/download.html).

> Note: `rover supergraph compose` is currently not available for Alpine Linux. You may track the progress for supporting this command on Alpine in [this issue](https://github.com/apollographql/rover/issues/537).

#### Windows PowerShell installer

```bash
iwr 'https://rover.apollo.dev/win/latest' | iex
```

To install a specific version of Rover (note the `v` prefixing the version number):

> Note: If you're installing Rover in a CI environment, it's best to target a specific version rather than using the latest URL, since future major breaking changes could affect CI workflows otherwise.

```bash
iwr 'https://rover.apollo.dev/win/v0.1.1' | iex
```

#### npm installer

Rover is distributed on npm for easy integration with your JavaScript projects.

##### devDependency install

If you'd like to install `rover` as a `devDependency` in your JavaScript project, you can run `npm i --save-dev @apollo/rover`. You can then call `rover` directly in your `package.json` [scripts](https://docs.npmjs.com/cli/v6/using-npm/scripts), or you can run `npx rover` in your project directory to execute commands.

##### Manual download and install

If you'd like to call `rover` from any directory on your machine, you can run `npm i -g @apollo/rover`.

Note: Unfortunately if you've installed `npm` without a version manager such as `nvm`, you may have trouble with global installs. If you encounter an `EACCES` permission-related error while trying to install globally, DO NOT run the install command with `sudo`. [This support page](https://docs.npmjs.com/resolving-eacces-permissions-errors-when-installing-packages-globally) has information that should help to resolve this issue.

#### Without curl

You can also [download the binary for your operating system](https://github.com/apollographql/rover/releases) and manually add its location to your `PATH`.

## Contributions

See [this page](https://go.apollo.dev/r/contributing) for info about contributing to Rover.

## License

This project is licensed under the MIT License ([LICENSE] or http://opensource.org/licenses/MIT).

[LICENSE]: https://github.com/apollographql/rover/blob/main/LICENSE

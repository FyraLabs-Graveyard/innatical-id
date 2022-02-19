# Innatical ID Integration for tauOS

Monorepo containing the GUI and daemon for Innatical ID integration

## Building

You'll need the following dependencies:

> *Note*: This dependency list is the names searched for by `pkg-config`. Depending on your distribution, you may need to install other packages (for example, `gtk4-devel` on Fedora)

- `meson`
- `valac`
- `rustc`
- `cargo`
- `gtk4`

Run `meson build` to configure the build environment. Change to the build directory and run `ninja test` to build and run automated tests.

```bash
$ meson build --prefix=/usr
$ cd build
$ ninja test
```

For debug messages on the GUI application, set the `G_MESSAGES_DEBUG` environment variable, e.g. to `all`:

```bash
G_MESSAGES_DEBUG=all ./gui/innatical-id-settings
```

## Installing

To install, use `ninja install`, then execute the GUI with `innatical-id-settings`. The CLI and daemon can be executed with `innatical-id-daemon`.

```bash
$ sudo ninja install
$ innatical-id-settings
$ innatical-id-daemon --help
```

> The `innatical-id-daemon` package is automatically managed by the GUI and SystemD, however can be configured with CLI flags.

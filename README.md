# Galactic Marauders

Galactic marauders is a Space Invaders like game written in rust using the bevy framework.

## Running Galactic Marauders

> cd ~/Galactic-Marauders
>
> cargo build
>
> cargo run

## Tests

> cd ~/Galactic-Marauders
>
> cargo test

## Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>
>
> install the packages
>
> "alsa-sys"
>
> "libudev-sys"

For apt

> sudo apt install librust-alsa-sys-dev librust-libudev-sys-dev

For dnf

> sudo dnf install rust-alsa-sys-devel rust-libudev-sys-devel

Or

> bash setup_dependencies.sh

To remove the additional dependencies run

> remove the packages
>
> "alsa-sys"
>
> "libudev-sys"

For apt

> sudo apt purge librust-alsa-sys-dev librust-libudev-sys-dev

For dnf

> sudo dnf remove rust-alsa-sys-devel rust-libudev-sys-devel

Or

> bash remove_dependencies.sh

Please use good op-sec habits and check the contents of both .sh files before running them as these will modify your system

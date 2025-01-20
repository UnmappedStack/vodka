# Vodka
x86_64 to ARM64 assembly "compiler", in Rust.

<img alt="Image of a cartoon bottle of vodka" src="https://images.vexels.com/media/users/3/144248/isolated/preview/d66ecac30244fedd1e36e415f3bc904e-russia-vodka-illustration.png" width="50%">

## What is this?
Simply put, this is a project that aims to be able to convert x86_64 assembly (GAS, intel syntax) to ARM64 assembly. This has one major goal: be able to convert already-compiled programs for x86_64 to ARM64 programs.

This is done by disassembling the program, using Vodka to convert it to ARM64 assembly, then reassembling and relinking it. This effectively allows you to run x86_64 programs on ARM64 computers.

## Usage
You'll need to already have `rustup` installed. To clone, build, and run the repo, run:
```sh
git clone https://github.com/UnmappedStack/vodka
cd vodka
cargo run
```
This will run a test x86 assembly file, `test.S`, and try to convert it to ARM64 assembly. Please note that Vodka is not yet ready to be properly used to convert full programs.

## Status
Vodka is still in extremely early stages. Realistically, I'm not even particularly sure if it'll *be able* to get to a usable point, because of issues such as different ABIs etc.

This is mostly a project for fun and for me to dip my toes in ARM64 assembly since at the moment I only know x86_64 assembly, so it may not be perfect.

## Why the name?
I see Vodka as a sort of "[Wine](https://www.winehq.org/) for ISAs", so I'm following their footpath in naming the project after an alcoholic drink in homage.

## License and contributions
Vodka is under the Mozilla Public License 2.0. See [LICENSE](LICENSE) for more information.

Additionally, Vodka is open to contributions, but please first open an issue before beginning work on the update or making a pull request, and explain what you want to change and why it's needed.

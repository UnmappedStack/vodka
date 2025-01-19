# Vodka
x86_64 to ARM64 assembly "compiler", in Rust.

![Image of a cartoon bottle of vodka](https://www.vexels.com/png-svg/preview/144248/russia-vodka-illustration)

## What is this?
Simply put, this is a project that aims to be able to convert x86_64 assembly (GAS, intel syntax) to ARM64 assembly. This has one major goal: be able to convert already-compiled programs for x86_64 to ARM64 programs.

This is done by disassembling the program, using Vodka to convert it to ARM64 assembly, then reassembling and relinking it. This effectively allows you to run x86_64 programs on ARM64 computers.

## Status
Vodka is still in extremely early stages, and cannot be used yet. Realistically, I'm not even particularly sure if it'll *be able* to get to a usable point, because of issues such as different ABIs etc.

This is mostly a project for fun and for me to dip my toes in ARM64 assembly since at the moment I only know x86_64 assembly, so it may not be perfect.

## Why the name?
I see Vodka as a sort of "[Wine](https://www.winehq.org/) for ISAs", so I'm following their footpath in naming the project after an alcoholic drink in homage.

## License and contributions
Vodka is under the Mozilla Public License 2.0. See [LICENSE](LICENSE) for more information.

Additionally, Vodka is open to contributions, but please first open an issue before beginning work on the update or making a pull request, and explain what you want to change and why it's needed.

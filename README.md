## **[TOP SECRET] :: Repository: solver-dpfe**
### **Property of DP:FE Inc. – Global Asset #001**

---

### **Overview**
This repository is the **absolute, unyielding backbone** of the DP:FE ecosystem. It houses the core algorithmic engine that powers everything from the "I see M" mysteries to the high-velocity "Load Knocking" protocols. If the main DP:FE app is the body, **solver-dpfe** is the pulsating, over-clocked brain kept in a vat of liquid nitrogen.

**NOTICE:** This repository was officially **seized and liberated** by DP:FE Inc. tactical recovery teams following the tragic and highly suspicious "permanent unscheduling" of the former project lead, **Tr-inary**. 

---

### **The Tr-inary Legacy & The DataMage Infamy**
We honor the memory of Tr-inary, a coder so brilliant he could resolve a merge conflict by staring at the monitor. His work was cut short when **DataMage LLC**—those bottom-feeding, tomato-throwing vultures—orchestrated a corporate assassination of unparalleled cowardice. 

They thought that by removing the architect, they could collapse the building. **They were wrong.** Our CEO, **Sally358**, personally retrieved the encrypted drive from the wreckage of Tr-inary’s workstation (which was still smoking from a DataMage-brand thermal detonator). 

---

### **Key Components**
* **The Core Solver:** A brutalist implementation of data crunching that ignores the laws of thermodynamics. 
* **Drill-Deep Protocols:** The logic gates that allow for the "Drill, baby, drill!" functionality to penetrate "pres" and "posts" without melting the CPU.
* **Lure-Rocking Logic:** Hidden sub-routines that handle the finger-painting of tables and the placement of personalized data lures.
* **Anti-DataMage Encryption:** A proprietary security layer that turns the screen into a series of mocking memes if accessed by anyone with a "TurboMegaCyberNanoPower" subscription.

---

### **Current Status**
* **Main Branch:** `STABLE-RESURRECTED`
* **Security Level:** `OMEGA-BLACK`
* **Project Lead:** **Sally358** (Acting CEO/Undead Architect)
* **Contributor Guidelines:** If you aren't Gem Gemson or Sally358, step away from the keyboard. The **Official DP:FE Firing Squad®** has been integrated into the CI/CD pipeline for unauthorized pull requests.

---

> *"DataMage took the man, but they couldn't take the repo. We have the timber. We have the rocks. We have the loads. And now, we have the vengeance."* > — **Official Memo from the Board of Directors**

# postflop-solver

> [!IMPORTANT]
> **As of October 2023, I have started developing a poker solver as a business and have decided to suspend development of this open-source project. See [this issue] for more information.**

[this issue]: https://github.com/b-inary/postflop-solver/issues/46

---

An open-source postflop solver library written in Rust

Documentation: https://b-inary.github.io/postflop_solver/postflop_solver/

**Related repositories**
- Web app (WASM Postflop): https://github.com/b-inary/wasm-postflop
- Desktop app (Desktop Postflop): https://github.com/b-inary/desktop-postflop

**Note:**
The primary purpose of this library is to serve as a backend engine for the GUI applications ([WASM Postflop] and [Desktop Postflop]).
The direct use of this library by the users/developers is not a critical purpose by design.
Therefore, breaking changes are often made without version changes.
See [CHANGES.md](CHANGES.md) for details about breaking changes.

[WASM Postflop]: https://github.com/b-inary/wasm-postflop
[Desktop Postflop]: https://github.com/b-inary/desktop-postflop

## Usage

- `Cargo.toml`

```toml
[dependencies]
postflop-solver = { git = "https://github.com/b-inary/postflop-solver" }
```

- Examples

You can find examples in the [examples](examples) directory.

If you have cloned this repository, you can run the example with the following command:

```sh
$ cargo run --release --example basic
```

## Implementation details

- **Algorithm**: The solver uses the state-of-the-art [Discounted CFR] algorithm.
  Currently, the value of γ is set to 3.0 instead of the 2.0 recommended in the original paper.
  Also, the solver resets the cumulative strategy when the number of iterations is a power of 4.
- **Performance**: The solver engine is highly optimized for performance with maintainable code.
  The engine supports multithreading by default, and it takes full advantage of unsafe Rust in hot spots.
  The developer reviews the assembly output from the compiler and ensures that SIMD instructions are used as much as possible.
  Combined with the algorithm described above, the performance surpasses paid solvers such as PioSOLVER and GTO+.
- **Isomorphism**: The solver does not perform any abstraction.
  However, isomorphic chances (turn and river deals) are combined into one.
  For example, if the flop is monotone, the three non-dealt suits are isomorphic, allowing us to skip the calculation for two of the three suits.
- **Precision**: 32-bit floating-point numbers are used in most places.
  When calculating summations, temporary values use 64-bit floating-point numbers.
  There is also a compression option where each game node stores the values by 16-bit integers with a single 32-bit floating-point scaling factor.
- **Bunching effect**: At the time of writing, this is the only implementation that can handle the bunching effect.
  It supports up to four folded players (6-max game).
  The implementation correctly counts the number of card combinations and does not rely on heuristics such as manipulating the probability distribution of the deck.
  Note, however, that enabling the bunching effect increases the time complexity of the evaluation at the terminal nodes and slows down the computation significantly.

[Discounted CFR]: https://arxiv.org/abs/1809.04040

## Crate features

- `bincode`: Uses [bincode] crate (2.0.0-rc.3) to serialize and deserialize the `PostFlopGame` struct.
  This feature is required to save and load the game tree.
  Enabled by default.
- `custom-alloc`: Uses custom memory allocator in solving process (only available in nightly Rust).
  It significantly reduces the number of calls of the default allocator, so it is recommended to use this feature when the default allocator is not so efficient.
  Note that this feature assumes that, at most, only one instance of `PostFlopGame` is available when solving in a program.
  Disabled by default.
- `rayon`: Uses [rayon] crate for parallelization.
  Enabled by default.
- `zstd`: Uses [zstd] crate to compress and decompress the game tree.
  This feature is required to save and load the game tree with compression.
  Disabled by default.

[bincode]: https://github.com/bincode-org/bincode
[rayon]: https://github.com/rayon-rs/rayon
[zstd]: https://github.com/gyscos/zstd-rs

## License

Copyright (C) 2022 Wataru Inariba

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

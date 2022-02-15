# Contract Version

Automatically get host/repository information at compile-time and insert them into your contracts.

## Example

```bash
near view "dev-1644939364756-75367548411779" "version"
```

output:
```json
{
  "name": 'example-counter',
  "semver": '0.0.2',
  "git_sha": '5e9df2b96ded9a80c21c0609d27c36758d2bcfca',
  "git_datetime": '2022-02-15 12:03:35 -0300',
  "git_dirty": true,
  "cargo_features": 'default',
  "cargo_profile": 'release',
  "rustc_semver": '1.56.1',
  "rustc_llvm": '13.0',
  "rustc_sha": '59eed8a2aac0230a8b53e89d4e99d55912ba6b35'
}
```

## Usage

- Please see [contract-version-example](https://github.com/nearcomponents/contract-version-example). This [diff](https://github.com/nearcomponents/contract-version-example/commit/1a99e4e0156a973d679879550f68b0bd0779bcf2) splits a normal contract from one that intends to use this library.

### Build-Script

To get all the necessary information, you'll need a _build script file_ for your contract, which should compile and execute before the compilation of your contract itself.

So we depend on the library both at normal compilation-time, but also at that build-script time. Edit `Cargo.toml`, adding two sections:
```toml
[dependencies.contract-version]
git = "https://github.com/nearcomponents/contract-version.git"
rev = "6d452a4"

[build-dependencies.contract-version]
git = "https://github.com/nearcomponents/contract-version.git"
rev = "6d452a4"
```

Then add the `build.rs` file alongside your `Cargo.toml` file (not inside of `src/`):
```rust
//! Executed before the contract's compilation.

use contract_version::build;

fn main() {
    // generates the version information and set the env vars
    // that will be present at compile-time on the contract's
    // compilation
    build::create_version().set_env();
    // makes it so this build.rs step always runs
    build::setup_rerun();
    //
    // note: if the build.rs step panics, it may not trigger
    // it's automatic rerun, so you'd need to `touch build.rs`
    // to guarantee it's next rerun.
}
```

Now all the necessary info is being gathered before your code compiles.  
Note that this script will run for every time you `check`/`build` your code, preventing your contract's compilation from being cached, which should increase the compilation-time. To avoid this and re-enable compilation caching, you can comment comment-out the `build::setup_rerun()` line, and manually `touch build.rs` when you want the information to be generated anew. This `touch` command will update the time in which the file was edited, and this will trigger the build-script to be re-run on for the next contract compilation.

### Contract Method

The remaining part is actually creating a method on your contract to get that information created at compile-time and show it.

You can create a file `src/version.rs`:
```rust
use crate::Counter;
use contract_version::{version_from_env, IVersion, Version};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CounterContract;

#[near_bindgen]
impl IVersion for Counter {
    fn version(&self) -> Version {
        version_from_env!()
    }
}
```
Please note that you should change everything named `Counter` to your contract's name. So if your contract is called `X`, then `Counter` should become `X`, and `CounterContract` should become `XContract`.

This will already be creating and exposing the `version` function. All that remains is adding that file as a module in your contract. On `lib.rs`:
```rust
// ..
mod version;
// ..
```

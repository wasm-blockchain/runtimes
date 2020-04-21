# Runtimes
Catalog of Wasm runtimes together with the corresponding tests and benchmarks. This catalog is specifically focused on the runtimes used or potentially useful for the blockchains.

## Features

<table>
  <tr>
    <th>Runtime</th>
    <th colspan="3">Wasmer</th>
    <th colspan="2">Wasmtime</th>
    <th> Wasmi </th>
  </tr>
  <tr>
    <td><b>Backend</b></td>
    <td><b>Singlepass</b></td>
    <td><b>Cranelift</b></td>
    <td><b>LLVM</b></td>
    <td><b>Cranelift</b></td>
    <td><b>Lightbeam</b></td>
    <td></td>
  </tr>
  <tr>
    <td>Type</td>
    <td>Non-optimizing<br/>compiler</td>
    <td>Optimizing<br/>compiler</td>
    <td>Optimizing<br/>compiler</td>
    <td>Optimizing<br/>compiler</td>
    <td>Non-optimizing<br/>compiler</td>
    <td>Interpreter</td>
  </tr>
  <tr>
    <td>WASI</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:x:</td>
  </tr>
  <tr>
    <td>Metering<sup>*</sup></td>
    <td>:white_check_mark:</td>
    <td>:x:</td>
    <td>:white_check_mark:</td>
    <td>:repeat:<sup><a href="#wasmtime-metering">6</a></sup></td>
    <td>:x:</td>
    <td>:white_check_mark:<a href="https://lib.rs/crates/metered_wasmi">metered_wasmi</a></td>
  </tr>
  <tr>
    <td>SIMD</td>
    <td>:x:</td>
    <td>:x:</td>
    <td>:white_check_mark:</td>
    <td>:repeat:</td>
    <td>:question:</td>
    <td>:x:</td>
  </tr>
  <tr>
    <td>Multi-value return</td>
    <td>:repeat:</td>
    <td>:repeat:</td>
    <td>:repeat:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:question:</td>
  </tr>
</table>

* :white_check_mark: -- yes
* :repeat: -- in progress
* :question: -- unknown
* :x: -- no

### Metering

Currently there are three implementations of the metering:
* [pwasm-utils by Parity](https://crates.io/crates/pwasm-utils), which is a code transformation tool that injects metering code into a Wasm file;
* Wasmer runtime is able to perform gas metering by injecting low-level metering code upon Wasm compilation;
* [wasm-metering by Ewasm](https://github.com/ewasm/wasm-metering) -- currently orphaned, gas metering injection.

Additionaly Wasmi has its own version with embedded metering functionality, see <a href="https://lib.rs/crates/metered_wasmi">metered_wasmi</a>.

Both pwasm-utils by Parity and wasm-metering by Ewasm are code transformation tools that can be combined with any Wasm runtime, e.g. someone can inject gas metering into the code before passing it to a runtime for execution.

## OS Support

<table>
  <tr>
    <th>Runtime</th>
    <th colspan="3">Wasmer</th>
    <th colspan="2">Wasmtime</th>
    <th> Wasmi </th>
  </tr>
  <tr>
    <td><b>Backend</b></td>
    <td><b>Singlepass</b></td>
    <td><b>Cranelift</b></td>
    <td><b>LLVM</b></td>
    <td><b>Cranelift</b></td>
    <td><b>Lightbeam</b></td>
    <td></td>
  </tr>
  <tr>
    <td>GNU Linux</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:white_check_mark:</td>
  </tr>
  <tr>
    <td>MacOS</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:white_check_mark:</td>
  </tr>
  <tr>
    <td>Windows NT</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:x:<a href="https://github.com/wasmerio/wasmer/issues/347">#347</a></td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:question:</td>
  </tr>
</table>

## Testing

<table>
  <tr>
    <th>Runtime</th>
    <th colspan="3">Wasmer</th>
    <th colspan="2">Wasmtime</th>
    <th> Wasmi </th>
  </tr>
  <tr>
    <td><b>Backend</b></td>
    <td><b>Singlepass</b></td>
    <td><b>Cranelift</b></td>
    <td><b>LLVM</b></td>
    <td><b>Cranelift</b></td>
    <td><b>Lightbeam</b></td>
    <td></td>
  </tr>
  <tr>
    <td>Spec tests</td>
    <td>:white_check_mark:<sup><a href="#wasmer-spectests">2</a></sup></td>
    <td>:white_check_mark:<sup><a href="#wasmer-spectests">2</a></sup></td>
    <td>:white_check_mark:<sup><a href="#wasmer-spectests">2</a></sup></td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:question:</td>
  </tr>
  <tr>
    <td>cargo-fuzz (libfuzzer)</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:x:</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:white_check_mark:<sup><a href="#wasmi">4</a></sup></td>
  </tr>
  <tr>
    <td>honggfuzz</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:x:</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:question:</td>
    <td>:question:</td>
    <td>:white_check_mark:<sup><a href="#wasmi">4</a></sup></td>
  </tr>
  <tr>
    <td>aflfuzz</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:x:</td>
    <td>:white_check_mark:<sup><a href="#wasmer-fuzz">3</a></sup></td>
    <td>:question:</td>
    <td>:question:</td>
    <td>:x:</td>
  </tr>
</table>

Note, additionally to fuzz tests provided by Wasmer [Trail of Bits](https://www.trailofbits.com/) performed volatility study of Wasmer and have created afl test that you can find in [wasmer-afl](https://github.com/wasm-blockchain/runtimes/blob/master/wasmer-afl) directory.


### Spec tests
Both Wasmer and Wasmi use official test suite to verify compatibility with the Wasm spec.

### Additional properties
Additionaly Wasmtime+Cranelift supports the following, according to [@tschneidereit](https://github.com/tschneidereit/):
* Debugging in gdb/lldb
* Openly governed
* License gives a patent grant

## Footnotes
<p><a name="wasmer-features" href="https://docs.wasmer.io/ecosystem/wasmer/wasmer-features#support-of-features-by-backend">1: Wasmer support of features by backend</a></p>
<p><a name="wasmer-spectests" href="https://github.com/wasmerio/wasmer/tree/master/tests/spectests">2: Wasmer spec tests</a></p>
<p><a name="wasmer-fuzz" href="https://github.com/wasmerio/wasm-fuzz">3: Wasmer fuzz tests</a></p>
<p><a name="wasmi" href="https://github.com/paritytech/wasmi">4: Wasmi homepage</a></p>
<p><a name="wasm-testsuite" href="https://github.com/WebAssembly/testsuite">5: Wasm official test suite</a></p>
<p><a name="wasmtime-metering">6: There are rumors of metering projects for Cranelift/Wasmtime. They might not be public, according to <a href="https://github.com/tschneidereit/">@tschneidereit</a></a></p>

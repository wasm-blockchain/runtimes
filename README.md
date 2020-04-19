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
    <td>:question:</td>
    <td>:question:</td>
    <td>:x:</td>
  </tr>
  <tr>
    <td>Metering<sup>*</sup></td>
    <td>:white_check_mark:</td>
    <td>:x:</td>
    <td>:white_check_mark:</td>
    <td>:x:</td>
    <td>:x:</td>
    <td>:white_check_mark:<a href="https://lib.rs/crates/metered_wasmi">metered_wasmi</a></td>
  </tr>
  <tr>
    <td>SIMD</td>
    <td>:x:</td>
    <td>:x:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:question:</td>
    <td>:x:</td>
  </tr>
  <tr>
    <td>Multi-value return</td>
    <td>:repeat:</td>
    <td>:repeat:</td>
    <td>:repeat:</td>
    <td>:question:</td>
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
1. [pwasm-utils by Parity](https://crates.io/crates/pwasm-utils), which is a code transformation tool that injects metering code into a Wasm file;
2. Wasmer runtime is able to perform gas metering by injecting low-level metering code upon Wasm compilation;
3. [wasm-metering by Ewasm](https://github.com/ewasm/wasm-metering),  

Additionaly Wasmi has its own version with embedded metering functionality, see <a href="https://lib.rs/crates/metered_wasmi">metered_wasmi</a>.

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
    <td>:question:</td>
    <td>:question:</td>
    <td>:white_check_mark:</td>
  </tr>
  <tr>
    <td>Mac OSX</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:question:</td>
    <td>:question:</td>
    <td>:white_check_mark:</td>
  </tr>
  <tr>
    <td>Windows NT</td>
    <td>:white_check_mark:</td>
    <td>:white_check_mark:</td>
    <td>:x:<a href="https://github.com/wasmerio/wasmer/issues/347">#347</a></td>
    <td>:question:</td>
    <td>:question:</td>
    <td>:question:</td>
  </tr>
</table>

----

## Footnotes

<a name="wasmer-features">1</a>: [Wasmer support of features by backend](https://docs.wasmer.io/ecosystem/wasmer/wasmer-features#support-of-features-by-backend)

### 0.8.8 ###
* :star: Modernize lints and CI workflows. See [#129](https://github.com/stepfunc/oo_bindgen/pull/129).
* :star: Add RID for aarch64-pc-windows-msvc in .NET backend

### 0.8.7 ###
* :star: Sets assembly version information to match the schema in C# backend

### 0.8.6 ###
* :star: Improve Java native library loading. See [#124](https://github.com/stepfunc/oo_bindgen/pull/124).

### 0.8.5 ###
Release was yanked

### 0.8.4 ###
* :star: Fix lints. See [#120](https://github.com/stepfunc/oo_bindgen/pull/120).

### 0.8.3 ###
* :book: Properly document async methods in C# and Java. See [#115](https://github.com/stepfunc/oo_bindgen/pull/115).
* :book: Add javadoc to builder methods in Java. See [#117](https://github.com/stepfunc/oo_bindgen/pull/117).

### 0.8.2 ###
* :star: Allow strings in universal structs. See [#113](https://github.com/stepfunc/oo_bindgen/pull/113).

### 0.8.0 ###
* :wrench: Remove future interface generation in favor of using a companion helper crate.

### 0.7.1 ###
* :wrench: Future interface uses lifetime on value. See [#110](https://github.com/stepfunc/oo_bindgen/pull/110).

### 0.7.0 ###
* :wrench: Wrap raw C future interface in a drop-safe Promise. See [#109](https://github.com/stepfunc/oo_bindgen/pull/109).

### 0.6.3 ###
* :wrench: Don't emit C interface initializers. See [#107](https://github.com/stepfunc/oo_bindgen/pull/107).

### 0.6.2 ###
* :wrench: Small tweaks to generate code that makes the newer versions of clippy happy.

### 0.6.1 ###
* :wrench: Java: Throw exception instead of exiting when library cannot be loaded. See [#104](https://github.com/stepfunc/oo_bindgen/pull/104).

### 0.6.0 ###
* :wrench: Integrate crates. See [#99](https://github.com/stepfunc/oo_bindgen/pull/99).
* :star: Change license to MIT OR Apache-2.0.

### 0.5.1 ###
* :star: CMake generator now indicates C++14 features required for C++ targets.

### 0.5.0 ###
* :star: Use a configuration file to control which FFI/JNI shared libraries are packaged for each binding language. See [#97](https://github.com/stepfunc/oo_bindgen/pull/97).

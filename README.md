![Step Function I/O](./sfio_logo.png)

# oo-bindgen

![CI](https://github.com/stepfunc/oo_bindgen/workflows/CI/badge.svg)

Object-oriented binding generator for Rust.

## License

Refer to `License.txt` for the terms of the non-commercial license.  This software is "source available", 
but is not "open source". You must purchase a commercial license to use this software for profit.

## How it works

- First, you write your Rust library without thinking about bindings.
- Then, you write a C FFI to your Rust library, taking into account how object-
  oriented languages will interact with it. You also make sure to protect as
  much as possible the interface between your Rust library and the outside C
  world
- You define a general object-oriented "schema" that uses the C FFI to interact
  with your library.
- You generate the bindings in the target languages using generators that reads
  the previously defined "schema" and generate easy-to-use, idiomatic and
  portable code.
- You write unit tests in the generated languages to make sure everything works
  as expected.

## Directories

- `oo-bindgen`: main library to build an object-oriented representation of your
  library.
- `generators`: different generators that takes a library defined using
  `oo-bindgen` to create easy-to-use bindings.
- `tests`: contains an example `foo-ffi` library with the associated
  `foo-bindings` object-oriented library definition. It builds the same library
  in each supported language. Each language has extensive unit tests written to
  check that the generated bindings work as expected.
- `ci-script`: a library for uniform CI scripts of projects

## Using the compiled bindings

For every project, a GitHub Actions pipeline generates bindings that are ready
to use on Windows x64 and Linux distributions based on glibc.

### C bindings

A minimal `CMakeLists.txt` to compile with a library generated by oo-bindgen is
the following:

```cmake
cmake_minimum_required(VERSION 3.8)

project(my_awesome_project LANGUAGES C)

# Find the foo library generated by oo-bindgen
set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_LIST_DIR}/foo/cmake)
find_package(foo REQUIRED)

# Add your awesome project with a dependency to foo
add_executable(my_awesome_project main.c)
target_link_libraries(my_awesome_project PRIVATE foo)

# Copy the DLL/.so after build
add_custom_command(TARGET my_awesome_project POST_BUILD 
    COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_FILE:foo> $<TARGET_FILE_DIR:my_awesome_project>
)
```

### .NET bindings

- Create a new [local NuGet feed](https://docs.microsoft.com/en-us/nuget/hosting-packages/local-feeds).
  To do this, create an empty directory somewhere then run `nuget sources add -Name my-nuget-feed -Source /my-nuget-feed`.
- Add the generated NuGet package to it using `nuget add foo.0.1.0.nupkg -Source /my-nuget-feed`.
- In your project, add a dependency to the package. You can do `dotnet add
  MyAwesomeProject package foo` or add it using Visual Studio interface. It
  should add a line in your `.csproj` like the following: `<PackageReference Include="foo" Version="0.1.0" />`

### Java bindings

- Install the JAR to your local Maven repository with
  `mvn org.apache.maven.plugins:maven-install-plugin:3.0.0-M1:install-file -D"file=foo-0.1.0.jar"`
- Add the dependency in your project with

```xml
<dependency>
    <groupId>io.stepfunc</groupId>
    <artifactId>foo</artifactId>
    <version>0.1.0</version>
</dependency>
```

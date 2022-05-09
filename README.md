# Input Helper

Generic types and tools for unifying input libraries.

The main job of Input Helper is to provide a single, unified way to process input from any input framework.

### The Problem

Most libraries that take user input do so in a library-specific way.  In order to use the library, you must write 
library-specific code.

For example: You can't switch from Winit to SDL, or Gilrs to SDL, or any other library without having to rewrite all 
your input code. This can either couple your project to specific framework(s), or force you to waste time duplicating 
code.  In the case of using multiple input libraries, such as pairing Winit and Gilrs you must handle each framework 
separately.

#### The Solution

Library-specific input events can be converted to `InputEvents`, and sent through Input Helper using 
`InputHelper::send()`.  Then your application code can be written against Input Helper.  If you ever need to switch 
your input system, then there's no need to rewrite your business logic.  This also allows you to process all input code
in a single place regardless of its source.

Input Helper will provide default integration functions for common frameworks such as Winit, SDL, and Gilrs.  

### Project Status

Input Helper is still in early alpha, so expect bugs, missing features, changing APIs, and other spooky stuff until
release 1.0.

## Contribution

### License

Input Helper is licensed under either

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.


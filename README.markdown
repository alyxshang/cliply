# CLIPLY :minidisc:

***Making CLI applications in Rust easy. :minidisc:***

![GitHub CI](https://github.com/alyxshang/cliply/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

This repository contains the source code for a Rust crate that makes building CLI applications in Rust fast and easy.

## INSTALLATION :inbox_tray:

To use ***Cliply*** in your Rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
cliply = { git = "https://github.com/alyxshang/cliply", tag = "v.0.1.0" }
```

To find out exactly how to use the library, please refer to the "Example" section below or clone this repository and run the command `cargo doc --open` from the repository's root to view the API documentation.

## EXAMPLE :sparkles:

This code snippet holds an example showing a simple example of how to use Cliply to build a small CLI app. This app, allowing the user to print out greetings, has the following flags:

- A version flag. Can be called with any of the following options: `--version`, `-v`, `version`.
- A flag to print usage information. This flag can be called with any of the following options: `--help`, `-h`, `help`.
- A flag to print a generic greeting. This flag can be called with any of the following options: `--greet`, `-g`, `greet`.
- A flag to print a customized greeting. This flag can be called with any of the following options: `--cgreet DATA`, `-c DATA`, `cgreet DATA`. The placeholder `DATA` represents the name of the person to be greeted.

```Rust
/*
Cliply Example by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// Importing the error
/// struct to handle any
/// errors.
use cliply::CliplyError;

/// Main point of
/// entry for the 
/// Rust compiler.
pub fn main() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut my_app: App = App::new(
        &"Test App",
        &"0.1.0",
        &"Alyx Shang"
    );

    // Adding a greeting without data. Note the use of "false".
    my_app.add_arg(
        &"greet",
        &" generic greeting for the user", 
        &false
    );

    // Adding a greeting with data. Note the use of "true".
    my_app.add_arg(
        &"cgreet", 
        &"custom greeting for the user", 
        &true
    );

    // Was the version flag used?
    if my_app.version_is() {
        println!("{}", my_app.version_info());
    }

    // Was the help flag used?
    else if my_app.help_is() {
        println!("{}", my_app.help_info());
    }

    // Was the "greet" flag used?
    else if my_app.arg_was_used(&"greet") {
        println!("Hello World!");
    }

    // Was the "cgreet" flag used? Note the use of the `Result`!
    else if my_app.arg_was_used(&"cgreet") {
        let arg_data: Result<String, CliplyError> = my_app.get_arg_data(&"cgreet");
        match arg_data {
            Ok(x) => {
                println!("Hello, {}!", x);
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        }
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", my_app.help_info());
    }
    
}
```

## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Cliply :minidisc:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).

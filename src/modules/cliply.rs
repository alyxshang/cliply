/*
Cliply by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the environment API.
use std::env;

/// Importing the "get_index"
/// function from the "coutils"
/// crate for getting
/// vector indexes.
use coutils::get_index;

/// Importing the "has_index"
/// function to check whether
/// a vector has a certain index.
use coutils::has_index;

/// Importing the "has_item"
/// function to check whether
/// a vector contains a certain
/// item.
use coutils::has_item;

/// Importing the "clean_split"
/// function from the "coutils"
/// module to split strings.
use coutils::clean_split;

/// Importing this crate's
/// error structure.
use super::errors::CliplyError;

/// Importing the map API.
use std::collections::HashMap;

/// A structrue to hold information
/// on CLI arguments added to
/// an application.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArgData {
    pub help: String,
    pub data: bool
}

/// Implementing functions
/// for the `ArgData` structure.
impl ArgData {

    /// Implements a generic function for creating
    /// new instances of the `ArgData` structure.
    pub fn new(help: &str, data: &bool) -> ArgData {
        ArgData {
            help: help.to_string(),
            data: data.to_owned()
        }
    }

}

/// A struct for
/// a CLI app that stores
/// all relevant information
/// about the application.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct App{
    pub name: String,
    pub version: String,
    pub author: String,
    pub args: HashMap<String, ArgData>
}

/// Implementing functions
/// for the `App` structure.
impl App{

    /// A generic function for
    /// creating a new instance of the `App`
    /// structure.
    pub fn new(name: &str, version: &str, author: &str) -> App {
        let args: HashMap<String, ArgData> = HashMap::new();
        App {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            args: args
        }
    }

    /// Checks whether different variations of an
    /// argument were used and returns a boolean
    /// to that effect.
    pub fn arg_was_used(&self, arg: &str) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_first_letter: &String = &clean_split(&arg.to_string(), &String::from(""))[1];
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_minus_arg: String = format!("--{}", arg);
        if args.contains(&minus_arg) || 
           args.contains(&minus_minus_arg)||
           args.contains(&arg.to_string())
        {
            result = true;
        }
        else {}
        result
    }

    /// Adds an argument to the argument pool.
    /// If you'd like to accept data for an argument,
    /// set the "data" flag to either `true` or `false`.
    /// Returns nothing.
    pub fn add_arg(
        &mut self, 
        name: &str, 
        help: &str, 
        data: &bool,
    ) -> () {
        let arg_data: ArgData = ArgData::new(help, data);
        self.args.insert(name.to_string(), arg_data);
    }

    /// Retrieves the command line data for an argument if it was supplied.
    /// A `Result` type is returned.
    pub fn get_arg_data(&self, name: &str) -> Result<String, CliplyError> {
        let args: Vec<String> = env::args().collect();
        let mut result: &String = &String::from("");
        let arg_first_letter: &String = &clean_split(
            &name.to_string(), 
            &String::from("")
        )[1];
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_minus_arg: String = format!("--{}", name);
        if self.args[name].data && args.contains(&minus_arg){
            let next_pos: usize;
            if has_item(
                &args, 
                &minus_arg
            )
            {
                next_pos = match get_index(&args, &minus_arg){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("\"{}\" not found in arguments.", &minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else if self.args[name].data && args.contains(&minus_minus_arg) {
            let next_pos: usize;
            if has_item(
                &args, 
                &minus_minus_arg
            )
            {
                next_pos = match get_index(&args, &minus_minus_arg){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("\"{}\" not found in arguments.", &minus_minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &minus_minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else if self.args[name].data && args.contains(&name.to_owned()) {
            let next_pos: usize;
            if has_item(
                &args, 
                &name.to_string()
            )
            {
                next_pos = match get_index(&args, &name.to_string()){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("\"{}\" not found in arguments.", &name);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &name);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else {}
        Ok(result.to_string())
    }

    /// Checks whether version information on
    /// the application was requested. Returns
    /// a boolean to this effect.
    pub fn version_is(&self) -> bool {
        let res: &bool = &self.arg_was_used("version");
        *res
    }

    /// Checks whether usage information on
    /// the application was requested. Returns
    /// a boolean to this effect.
    pub fn help_is(&self) -> bool {
        let res: &bool = &self.arg_was_used("help");
        *res
    }

    /// Returns a string with 
    /// the application's version info.
    pub fn version_info(&self) -> String {
        format!(
            "{} v.{}\nby {}.", 
            &self.name, 
            &self.version, 
            &self.author
        )
    }
    
    /// Returns a string with usage 
    /// information for the application.
    pub fn help_info(&self) -> String {
        let mut help_string_vec: Vec<String> = Vec::new();
        let help_short: &String = &String::from("h");
        let help_long: &String = &String::from("help");
        let help_help_msg: &String = &String::from("displays this message");
        let version_short: &String = &String::from("v");
        let version_long: &String = &String::from("version");
        let version_help_msg: &String = &String::from("displays app info");
        for (key,value) in &self.args {
            if value.data {
                let first_letter: &String = &clean_split(
                    &key, 
                    &String::from("")
                )[1];
                let command_help: String = format!("-{} --{} {} DATA  {}", first_letter, &key, &key, value.help);
                help_string_vec.push(command_help);
            }
            else {
                let first_letter: &String = &clean_split(
                    &key.to_string(), 
                    &String::from("")
                )[1];
                let command_help: String = format!("-{} --{} {}        {}", first_letter, &key, &key, value.help);
                help_string_vec.push(command_help);
            }
        }
        help_string_vec.push(
            format!("-{} --{} {}           {}", help_short, &help_long, &help_long, help_help_msg)
        );
        help_string_vec.push(
            format!("-{} --{} {}     {}", version_short, &version_long, &version_long, version_help_msg)
        );
        let help_string = help_string_vec.join("\n");
        help_string
    }
}

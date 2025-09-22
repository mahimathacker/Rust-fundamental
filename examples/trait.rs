#![allow(unused)]

//Trait : A type that can be replaced with another type in the parameters of a function
//Traits are  used to define multiple input parameters for a function
struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}
trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String {
        "Good luck".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("Compiling {} with Solidity", file_path) //format! is a macro that formats the string
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("Compiling {} with Vyper", file_path) //format! is a macro that formats the string
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

// Remove the semicolon at the end of the format! macro calls. In Rust, the last expression in a function (without a semicolon) becomes the return value.

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vyper = Vyper {
        version: "0.4".to_string(),
    };

    println!("Help: {}", sol.help());
    println!("Help: {}", vyper.help());

    println!("Compiled Solidity: {}", sol.compile("main.sol"));
    println!("Compiled Vyper: {}", vyper.compile("main.vy"));

    println!("Compiled Solidity: {}", compile(&sol, "main.sol"));
    println!("Compiled Vyper: {}", compile(&vyper, "main.vy"));
}

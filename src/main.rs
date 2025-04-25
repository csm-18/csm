//csm toolchain version
const VERSION: &str = "0.1.0";
fn main() {
    let args:Vec<String> = std::env::args().collect();

    
        if args.len() == 1 {
            println!("csm {VERSION}");
            println!("toolchain for csm programming language.");
            println!("for more info:\n csm --help");
        }else if args.len() == 2 {
            if &args[1] == "--version" || &args[1] == "-v" {
                println!("csm {VERSION}");
            }else if &args[1] == "--help" || &args[1] == "-h" {
                println!("csm commands:");
                println!(" 1. csm <no args>                          -> prints about message.");
                println!(" 2. csm --version                          -> prints csm version.");
                println!(" 3. csm --help                             -> prints this commands list.");
                println!(" 4. csm init                               -> initializes a new project in current directory.");
                println!(" 5. csm build                              -> build current directory(project).");
                println!(" 6. csm run                                -> run current directory(project).");
            }else if &args[1] == "init" {
                println!("csm: initializing a new project in current directory.");
                
            }
        }
    
}

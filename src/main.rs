//csm toolchain version
const VERSION: &str = "0.1.0";
fn main() {
    let args:Vec<String> = std::env::args().collect();

    for arg in &args{
        if args.len() == 1 {
            println!("csm {VERSION}");
            println!("toolchain for csm programming language.");
            println!("for more info:\n csm --help");
        }else if args.len() == 2 {
            if arg == "--version" || arg == "-v" {
                println!("csm {VERSION}");
            }else if arg == "--help" || arg == "-h" {
                println!("csm commands:");
                println!(" 1. csm <no args>                 -> prints about message.");
                println!(" 2. csm --version                 -> prints csm version.");
                println!(" 3. csm --help                    -> prints this commands list.");
                println!(" 4. csm main.csm -o <output-name> -> compiles to single exe binary with output name.");        
                println!(" 5. csm init                      -> initializes a new project in current directory.");
                println!(" 6. csm run                       -> run current directory(project).");
            }
        }
    }
}

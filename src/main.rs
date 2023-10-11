use clap::Parser;


/// CLI that makes pina colada
/// 

// #[derive(Debug, Clone)]
// enum Emotions {
//     Happy,
//     Sad,
//     Surprised,
// }

#[derive(Debug,Parser)]
struct Arguments {
    /// What ya say
    // Stuff
    message: String,
    // emotion: String,
}


fn main() {
    let args:Arguments =  Arguments::parse();
    
    println!("__{}__", "_".repeat(args.message.len()));
    println!("< {} >",args.message);
    println!("--{}--", "-".repeat(args.message.len()));
    println!("      \\");
    println!("       \\");
    println!("        \\");
    println!("      >(O v O)<");
    // match args.emotion {
    //     Emotions::Sad => println!("      >(O ^ O)<"),
    //     Emotions::Surprised => println!("      >(O o O)<"),
    //     _ => println!("      >(O v O)<"),
    // }
    
    dbg!(args.message);



    // println!(" _");
    // println!(" //\");
    // println!(" V  \");"
    // println!(" \\  \\_");
    // println!("\\,'.`-.");
    // println!(" |\\ `. `.       ");
    // println!(" ( \\  `. `-.     ");                   _,.-:\");
    // println!("    \\ \   `.  `-._             __..--' ,-';/");
    // println!("     \\ `.   `-.   `-..___..---'   _.--\' ,\'/");"
    // println!("    `. `.    `-._        __..--'    ,' /");
    // println!("        `. `-_     ``--..''       _.-' ,'");
    // println!("        `-_ `-.___        __,--'   ,'");
    // println!("            `-.__  `----"""    __.-'");
    // println!("                    `--..____..--'");
}

// Addons
// add flags to change expression
// explore terminal colors
// strings with emojis


use rpn::Interpreter;
use rustyline::Editor;

fn main() -> rustyline::Result<()> {
    print_title();
    
    let mut editor = Editor::<()>::new();
    loop {
        match Interpreter::new("rpn> ", &mut editor)?
            .expr()
            .unwrap_or_else(|err| {
                println!("Unable to parse input: {}", err);
                false
            }) {
            true => {
                break;
            }
            false => {
                continue;
            }
        }
    }
    Ok(())
}

fn print_title() {
   println!("\n ____________ _   _ ");
   println!(" | ___ \\ ___ \\ \\ | |");
   println!(" | |_/ / |_/ /  \\| |");
   println!(" |    /|  __/| . ` |");
   println!(" | |\\ \\| |   | |\\  |");
   println!(" \\_| \\_\\_|   \\_| \\_/");
   println!("      Version {} ", env!("CARGO_PKG_VERSION"));
   println!(" -------------------\n"); 
}

use calc::Interpreter;
use rustyline::Editor;

fn main() -> rustyline::Result<()> {
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

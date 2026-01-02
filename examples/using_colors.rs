use {
    supaterm::{
        style,
        self as st
    },
    
    std::io::{self, Write as _},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut term = st::Terminal::new(
        io::stdin().lock(),
        io::stdout().lock()
    )?;
    
    for i in 0..=255u8 {

        if i == 0 {
            term.write_all(b"\nStandard colors\n")?;
        }
        if i == 8 {
            term.write_all(b"\nStandard bright colors\n")?;
        }
        if i == 16 {
            term.write_all(b"\n6x6x6 color cube\n")?;
        }
        if i == 232 {
            term.write_all(b"\nGreyscale in 24 steps\n")?;
        }

        term.queue(style::SetForegroundColor(style::Color::ColorById(i)))?;

        write!(term, "{i:<4}")?;

        term.queue(style::ResetStyle)?;

        if i == 51 || i == 87 || i == 123 || i == 159 || i == 195 {
            term.write_all(b"\n")?;
        }
    }

    term.write_all(b"\n")?;
    
    Ok(())
}
use std::io::{self, Write};
use std::time::Duration;

use potoo_hedy_interpreter::{Host, Interpreter, Level, Turn, Vocabulary, parse};

struct CliHost {
    random_state: u64,
}

impl Host for CliHost {
    fn input(&mut self, prompt: &str) -> Result<String, String> {
        print!("{prompt}");
        io::stdout().flush().map_err(|error| error.to_string())?;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|error| error.to_string())?;
        Ok(input.trim_end_matches(['\r', '\n']).to_owned())
    }

    fn output(&mut self, text: &str) -> Result<(), String> {
        println!("{text}");
        Ok(())
    }

    fn random_index(&mut self, length: usize) -> Result<usize, String> {
        if length == 0 {
            return Err("cannot choose from an empty list".to_owned());
        }
        self.random_state ^= self.random_state << 13;
        self.random_state ^= self.random_state >> 7;
        self.random_state ^= self.random_state << 17;
        Ok((self.random_state as usize) % length)
    }

    fn sleep(&mut self, seconds: f64) -> Result<(), String> {
        let duration = Duration::try_from_secs_f64(seconds)
            .map_err(|error| format!("invalid sleep duration: {error}"))?;
        std::thread::sleep(duration);
        Ok(())
    }

    fn turtle_forward(&mut self, _steps: f64) -> Result<(), String> {
        Err("forward requires a graphical turtle host".to_owned())
    }

    fn turtle_turn(&mut self, _turn: Turn) -> Result<(), String> {
        Err("turn requires a graphical turtle host".to_owned())
    }

    fn turtle_color(&mut self, _color: &str) -> Result<(), String> {
        Err("color requires a graphical turtle host".to_owned())
    }

    fn turtle_clear(&mut self) -> Result<(), String> {
        Err("clear requires a graphical turtle host".to_owned())
    }

    fn play_note(&mut self, _note: &str) -> Result<(), String> {
        Err("play requires an audio host".to_owned())
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    if arguments.next().as_deref() != Some("--level") {
        return Err("usage: potoo-hedy --level 1..16 <source-file>".to_owned());
    }
    let level_number = arguments
        .next()
        .ok_or_else(|| "level is required".to_owned())?
        .parse::<u8>()
        .map_err(|_| "level must be an integer from 1 to 16".to_owned())?;
    let level = Level::try_from(level_number)?;
    let path = arguments
        .next()
        .ok_or_else(|| "source file is required".to_owned())?;
    if arguments.next().is_some() {
        return Err("unexpected extra argument".to_owned());
    }

    let source = std::fs::read_to_string(&path).map_err(|error| format!("{path}: {error}"))?;
    let vocabulary = Vocabulary::english();
    let program = parse(&source, level, &vocabulary).map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos() as u64
        | 1;
    Interpreter::new(CliHost { random_state: seed })
        .run(&program)
        .map_err(|error| error.to_string())?;
    Ok(())
}

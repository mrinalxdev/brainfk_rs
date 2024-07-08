use anyhow::Ok;
use anyhow::{Context, Result};
use std::env;
// use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process;

struct Lexer<R: Read> {
    source: R,
    location: Location,
    nxt_token: Option<Token>,
}

#[derive(Debug, Copy, Clone)]
struct Location {
    line: usize,
    column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Token {
    char: char,
    location: Location,
}

enum Operation {
    AddressRight,
    AddressLeft,
    Inc,
    Dec,
    Output,
    Input,
    JmpForward,
    JmpBackward,
}

impl<R> Lexer<R>
where
    R: Read,
{
    fn new(source: R) -> Self {
        Self {
            source,
            location: Location::default(),
            nxt_token: None,
        }
    }

    fn is_char_in_language(candidate: char) -> bool {
        let lang_chars = "<>+-.,[]";

        for char in lang_chars.chars() {
            if char == candidate {
                return true;
            }
        }

        return false;
    }

    fn slice_token(&mut self) -> Result<Option<Token>> {
        if self.nxt_token.is_some() {
            let token = self.nxt_token.take().expect("peeked token to be available");
            return Ok(Some(token));
        }

        let mut buf: [u8; 1] = [0; 1];
        let mut location: Location = self.location;

        while !Self::is_char_in_language(buf[0].into()) {
            location = self.location;
            let read_bytes = self
                .source
                .read(&mut buf)
                .context("read next byte from source")?;

            if read_bytes != 1 {
                return Ok(None);
            }
            self.location.column += 1;
            if buf[0] == '\n' as u8 {
                self.location.column = 1;
                self.location.line += 1;
            }
        }

        Ok(Some(Token {
            char: buf[0].into(),
            location,
        }))
        // todo!("create token")
    }

    fn nxt_token(&mut self) -> Result<Option<Token>> {
        if let Some(token) = self.nxt_token {
            return Ok(Some(token));
        }

        self.nxt_token = self
            .slice_token()
            .context("reading next token to peek at it")?;
        Ok(self.nxt_token)
    }
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args
        .split_first()
        .expect("expected to have at least the command in the args array");

    if args.len() < 1 {
        eprintln!("Usage : ");
        eprintln!("{command} <brainfuck_file>");
        process::exit(1);
    }

    let input = &args[0];

    println!("Opening brainfuck file {input} for execution");

    let reader = BufReader::new(
        File::open(input).with_context(|| format!("open file {input} for reading"))?,
    );
    let mut lexer = Lexer::new(reader);

    let nxt_token = lexer.nxt_token()?;
    println!("Peeked token {nxt_token:?} ");
    let slice_token = lexer.slice_token()?;
    println!("Sliced token {slice_token:?}");
    let slice_token = lexer.slice_token()?;
    println!("Sliced token {slice_token:?}");

    // while let Some(token) = lexer.slice_token().context(format!("read next token"))? {
    //     println!("{token:?}");
    // }

    println!(" ");

    Ok(())
}

use anyhow::{anyhow, Ok};
use anyhow::{Context, Result};
use std::env;
// use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read, Write};
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

#[derive(Debug, Clone)]
enum Instruction {
    AddressRight(usize),
    AddressLeft(usize),
    Inc(u8),
    Dec(u8),
    Output(usize),
    Input(usize),
    JmpForward(usize),
    JmpBackward(usize),
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

    fn count_while(&mut self, token: &Token) -> Result<usize> {
        let mut count: usize = 0;
        while let Some(candidate) = self.nxt_token()? {
            if candidate.char == token.char {
                let _ = self.slice_token();
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }
}

type Program = Vec<Instruction>;

#[derive(Default)]
struct Parser {
    forward_jmps: Vec<usize>,
    program: Program,
}

impl Parser {
    fn parse_instruction<R: Read>(
        &mut self,
        lexer: &mut Lexer<R>,
        token: &Token,
    ) -> Result<Instruction> {
        match token {
            Token { char: '<', .. } => Ok(Instruction::AddressLeft(1 + lexer.count_while(&token)?)),
            Token { char: '>', .. } => Ok(Instruction::AddressRight(1 + lexer.count_while(token)?)),
            Token { char: '[', .. } => {
                self.forward_jmps.push(self.program.len()); //This will locate the actual Instruction stream

                //Position will be backpatched once encountering the JmpBack..
                Ok(Instruction::JmpForward(0))
            }
            Token {
                char: ']',
                location: Location { line, column },
            } => {
                if let Some(target) = self.forward_jmps.pop() {
                    self.program[target] = Instruction::JmpForward(self.program.len());
                    return Ok(Instruction::JmpBackward(target + 1));
                } else {
                    return Err(anyhow!(
                        "Could not find corresponding jump for ] at {line}:{column}"
                    ));
                }
            }
            Token { char: ',', .. } => Ok(Instruction::Input(1 + lexer.count_while(token)?)),
            Token { char: '.', .. } => Ok(Instruction::Output(1 + lexer.count_while(token)?)),
            Token { char: '-', .. } => Ok(Instruction::Dec(
                ((1 + lexer.count_while(token)?) % 255) as u8,
            )),
            Token { char: '+', .. } => Ok(Instruction::Inc(
                ((1 + lexer.count_while(token)?) % 255) as u8,
            )),
            _ => unreachable!("No other token defined set is expected"),
        }
    }

    fn parse_program<R: Read>(&mut self, lexer: &mut Lexer<R>) -> Result<Program> {
        // let mut program = vec![];
        self.program = vec![];
        self.forward_jmps = vec![];
        while let Some(token) = lexer.slice_token()? {
            let instruction = self.parse_instruction(lexer, &token)?;
            self.program.push(instruction)
        }
        Ok(self.program.clone())
    }
}

struct Interpreter {
    program: Program,
    memory: Vec<u8>,
    addr: usize,
    instruction_ptr: usize,
}

impl Interpreter {
    fn new(program: Program) -> Self {
        Self {
            program,
            memory: vec![0; 64000],
            addr: 0,
            instruction_ptr: 0,
        }
    }
    fn run(&mut self) -> Result<()> {
        while self.instruction_ptr < self.program.len() {
            // println!(
            //     "{ip} : {instruction:?}",
            //     ip = self.instruction_ptr,
            //     instruction = self.program[self.instruction_ptr]
            // );
            // break;
            match self.program[self.instruction_ptr] {
                Instruction::AddressRight(count) => {
                    self.addr += count;
                    self.instruction_ptr += 1;
                }
                Instruction::AddressLeft(count) => {
                    self.addr -= count;
                    self.instruction_ptr += 1;
                }
                Instruction::Inc(count) => {
                    self.memory[self.addr] = self.memory[self.addr].wrapping_add(count);
                    self.instruction_ptr += 1;
                }
                Instruction::Dec(count) => {
                    self.memory[self.addr] = self.memory[self.addr].wrapping_sub(count);
                    self.instruction_ptr += 1;
                }
                Instruction::Output(count) => {
                    let mut stdout = std::io::stdout();
                    for _ in 0..count {
                        stdout
                            .write(&self.memory[self.addr..self.addr + 1])
                            .context("writing data to stdout")?;
                    }
                    stdout.flush().context("Flushing stdout output")?;
                    self.instruction_ptr += 1;
                }
                Instruction::Input(_count) => todo!(),
                Instruction::JmpForward(target) => {
                    if self.memory[self.addr] == 0 {
                        self.instruction_ptr = target;
                    } else {
                        self.instruction_ptr += 1;
                    }
                }
                Instruction::JmpBackward(target) => {
                    if self.memory[self.addr] != 0 {
                        self.instruction_ptr = target;
                    } else {
                        self.instruction_ptr += 1;
                    }
                }
            }
        }
        Ok(())
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

    let mut parser = Parser::default();

    //creating the parser
    let program = parser.parse_program(&mut lexer)?;
    let mut interpreter = Interpreter::new(program);
    interpreter.run()?;

    Ok(())
}

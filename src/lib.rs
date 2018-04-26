use std::io::{Read, Write};
use std::fs::File;

pub struct RustFk {
    d_ptr: usize,
    i_ptr: usize,
    data: Vec<u8>,
    commands: Vec<u8>
}

const INC_DPTR: u8 = '>' as u8;
const DEC_DPTR: u8 = '<' as u8;
const INC_DATA: u8 = '+' as u8;
const DEC_DATA: u8 = '-' as u8;
const WRITE:    u8 = '.' as u8;
const READ:     u8 = ',' as u8;
const JUMP_F:   u8 = '[' as u8;
const JUMP_B:   u8 = ']' as u8;


impl RustFk {
    /// Constructs a new RustFk interpreter
    /// 
    /// `d_size` is the amount of bytes to allocate for the tape
    pub fn new(d_size: usize, commands: Vec<u8>) -> RustFk {
        let data = vec![0; d_size];
        let d_ptr = d_size / 2;
        let i_ptr = 0;

        RustFk {
            d_ptr: d_ptr,
            i_ptr: i_ptr,
            data: data,
            commands: commands,
        }
    }

    /// Run the interpreter using the given streams of input and output
    pub fn run<R: Read, W: Write>(&mut self, input: &mut R, output: &mut W) -> Result<(), RustFkError> {
        loop {
            if self.i_ptr >= self.commands.len() {
                break
            }

            let next_cmd = self.commands[self.i_ptr];
            // println!("Feeding {}", next_cmd as char);
            self.feed(next_cmd, input, output)?;
            self.i_ptr += 1;
        }

        Ok(())
    }

    fn feed<R: Read, W: Write>(&mut self, cmd: u8, input: &mut R, output: &mut W) -> Result<(), RustFkError> {
        match cmd {
            INC_DPTR => {
                if self.d_ptr == self.data.len()-1 {
                    return Err(RustFkError{
                        msg: "cannot increment pointer, out of tape",
                    })
                }
                self.d_ptr += 1;
            },
            DEC_DPTR => {
                if self.d_ptr == 0 {
                    return Err(RustFkError{
                        msg: "cannot decrement pointer, out of tape",
                    })
                }
                self.d_ptr -= 1;
            },
            INC_DATA => {
                if self.data[self.d_ptr] == 255 {
                    self.data[self.d_ptr] = 0;
                } else {
                    self.data[self.d_ptr] += 1;
                }
            },
            DEC_DATA => {
                if self.data[self.d_ptr] == 0 {
                    self.data[self.d_ptr] = 255;
                } else {
                    self.data[self.d_ptr] -= 1;
                }
            },
            WRITE => {
                let buf = &self.data[self.d_ptr..self.d_ptr+1];
                if let Err(_) = output.write(buf) {
                    return Err(RustFkError { msg: "unable to write output" });
                }
            },
            READ => {
                // We take a one byte mutable slice from the data vector, and
                // read a byte into it from the input source
                let buf = &mut self.data[self.d_ptr..self.d_ptr+1];
                if let Err(_) = input.read_exact(buf) {
                    return Err(RustFkError { msg: "no input available" });
                }
            },
            JUMP_F => {
                if self.data[self.d_ptr] == 0 {
                    let mut brackets_seen = 0;
                    loop {
                        let cmd = self.commands[self.i_ptr];
                        if cmd == JUMP_B {
                            if brackets_seen == 1 {
                                break;
                            }
                            brackets_seen -= 1
                        } else if cmd == JUMP_F {
                            brackets_seen += 1;
                        }

                        self.i_ptr += 1;
                        if self.i_ptr == self.commands.len() {
                            return Err(RustFkError {
                                msg: "no matching ] found",
                            });
                        }
                    }
                }
            },
            JUMP_B => {
                if self.data[self.d_ptr] != 0 {
                    let mut brackets_seen = 0;
                    loop {
                        let cmd = self.commands[self.i_ptr];
                        if cmd == JUMP_F {
                            if brackets_seen == 1 {
                                break;
                            }
                            brackets_seen -= 1
                        } else if cmd == JUMP_B {
                            brackets_seen += 1;
                        }

                        if self.i_ptr == 0 {
                            return Err(RustFkError {
                                msg: "no matching [ found",
                            });
                        }
                        self.i_ptr -= 1;
                    }
                }
            },

            _ => {
                // Brainf*** ignores commands other than those specified
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RustFkError {
    msg: &'static str,
}

pub struct Config {
    filename: String,
}

impl Config {
    /// Create a new config to run the interpreter via command line
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config {
            filename,
        } )
    }

    /// Run the interpreter using stdin as input and stdout as output
    /// allocates 30000 bytes for the tape
    pub fn run(&self) -> Result<(), RustFkError> {
        let mut f = File::open(&self.filename).unwrap();

        let mut commands: Vec<u8> = vec![];
        f.read_to_end(&mut commands).unwrap();

        let mut input = std::io::stdin();
        let mut output = std::io::stdout();
        let mut interpreter = RustFk::new(30000, commands);
        interpreter.run(&mut input, &mut output)
    }
}
use std::io::Read;

pub struct RustFk {
    d_ptr: usize,
    i_ptr: usize,
    data: Vec<u8>,
    commands: Vec<u8>,
    input: std::io::Bytes<std::io::Stdin>,
}

const INC_DPTR: u8 = '>' as u8;
const DEC_DPTR: u8 = '<' as u8;
const INC_DATA: u8 = '+' as u8;
const DEC_DATA: u8 = '-' as u8;
const WRITE: u8 = '.' as u8;
const READ: u8 = ',' as u8;
const JUMP_F: u8 = '[' as u8;
const JUMP_B: u8 = ']' as u8;

impl RustFk {
    pub fn new(d_size: usize, commands: Vec<u8>) -> RustFk {
        let data = vec![0; d_size];
        let d_ptr = d_size / 2;
        let i_ptr = 0;

        RustFk {
            d_ptr: d_ptr,
            i_ptr: i_ptr,
            data: data,
            commands: commands,
            input: std::io::stdin().bytes(),
        }
    }

    pub fn run(&mut self) -> Result<(), RustFkError> {
        loop {
            if self.i_ptr >= self.commands.len() {
                break
            }

            let next_cmd = self.commands[self.i_ptr];
            // println!("Feeding {}", next_cmd as char);
            self.feed(next_cmd)?;
            self.i_ptr += 1;
        }

        Ok(())
    }

    fn feed(&mut self, cmd: u8) -> Result<(), RustFkError> {
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
                print!("{}", self.data[self.d_ptr] as char);
            },
            READ => {
                let ch = match self.input.next() {
                    Some(Ok(c)) => c,
                    _ => return Err(RustFkError { msg: "no input available" }),
                };
                
                self.data[self.d_ptr] = ch;
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
                // Brainf*** ignores comments other than those specified
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RustFkError {
    msg: &'static str,
}
use std::io::Read;

pub struct RustFk {
    d_ptr: usize,
    i_ptr: usize,
    data: Vec<u8>,
    commands: String,
}

impl RustFk {
    pub fn new(d_size: usize, commands: &str) -> RustFk {
        let data = vec![0; d_size];
        let d_ptr = d_size / 2;
        let i_ptr = 0;

        RustFk {
            d_ptr: d_ptr,
            i_ptr: i_ptr,
            data: data,
            commands: String::from(commands),
        }
    }

    pub fn run(&mut self) -> Result<(), RustFkInvalidCommand> {
        loop {
            if self.i_ptr >= self.commands.len() {
                break
            }

            let next_cmd = self.commands.as_bytes()[self.i_ptr];
            // let i = self.i_ptr;
            self.feed(next_cmd as char)?;
            // println!("{}: {} | {:?}", i, next_cmd as char, self.data);
            self.i_ptr += 1;
        }

        Ok(())
    }

    fn feed(&mut self, cmd: char) -> Result<(), RustFkInvalidCommand> {
        match cmd {
            '>' => {
                self.d_ptr += 1;
            },
            '<' => {
                self.d_ptr -= 1;
            },
            '+' => {
                if self.data[self.d_ptr] == 255 {
                    self.data[self.d_ptr] = 0;
                } else {
                    self.data[self.d_ptr] += 1;
                }
                // println!("d: {}", self.data[self.d_ptr]);
            },
            '-' => {
                if self.data[self.d_ptr] == 0 {
                    self.data[self.d_ptr] = 255;
                } else {
                    self.data[self.d_ptr] -= 1;
                }
                // println!("d: {}", self.data[self.d_ptr]);
            },
            '.' => {
                print!("{}", self.data[self.d_ptr] as char);
            },
            ',' => {
                let next_ch = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|x| x.ok())
                    .map(|byte| byte as u8); 
                if let Some(ch) = next_ch {
                    self.data[self.d_ptr] = ch;
                }
            },
            '[' => {
                if self.data[self.d_ptr] == 0 {
                    let b_cmd = self.commands.as_bytes();
                    loop {
                        self.i_ptr += 1;
                        if (b_cmd[self.i_ptr] as char) == ']' {
                            break;
                        }
                    }
                }
            },
            ']' => {
                if self.data[self.d_ptr] != 0 {
                    let b_cmd = self.commands.as_bytes();
                    loop {
                        self.i_ptr -= 1;
                        if (b_cmd[self.i_ptr] as char) == '[' {
                            break;
                        }
                    }
                }
            },

            _ => {
                return Err(RustFkInvalidCommand{})
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RustFkInvalidCommand;
use std::io::BufRead;
use std::os::unix::prelude::*;
use termios::*;

const PAGELEN: i32 = 24;

fn main() -> std::io::Result<()> {
    let mut _args = Vec::new();

    for arg in std::env::args().skip(1) {
        _args.push(arg);
    }

    if _args.len() == 0 {
        let stdin = std::io::stdin();
        do_more(stdin.lock())?;
    } else {
        for arg in &_args {
            let file = std::fs::File::open(arg)?;
            let buffered_reader = std::io::BufReader::new(file);
            do_more(buffered_reader)?;
        }
    }
    Ok(())
}

fn do_more<R>(reader: R) -> std::io::Result<()>
where
    R: BufRead,
{
    let mut num_of_lines = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        if num_of_lines == PAGELEN {
            let reply = see_more();
            if reply == 0 {
                break;
            }
            num_of_lines -= reply
        }
        println!("{}", line);
        num_of_lines += 1;
    }
    Ok(())
}

fn see_more() -> i32 {
    println!("more?");
    let stdin = std::io::stdin();
    let stdin_fd = stdin.as_raw_fd();
    let mut old_termios = Termios::from_fd(stdin_fd).unwrap();
    match tcgetattr(stdin_fd, &mut old_termios) {
        Ok(termios) => termios,
        Err(_error) => panic!("I Cannot open terminal attributes!!!"),
    };

    let mut new_termios = Termios::from_fd(stdin_fd).unwrap();
    new_termios.c_lflag = new_termios.c_lflag & !(ECHO | ICANON);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;
    match tcsetattr(stdin_fd, TCSAFLUSH, &mut new_termios) {
        Ok(termios) => termios,
        Err(_error) => panic!("I Cannot set terminal attributes!!!"),
    };

    let mut c = String::new();
    std::io::stdin()
        .read_line(&mut c)
        .ok()
        .expect("I Cannot read from stdin!!!");
    match tcsetattr(stdin_fd, TCSAFLUSH, &mut old_termios) {
        Ok(termios) => termios,
        Err(_error) => panic!("I Cannot set terminal attributes!!!"),
    };

    if c == "q\n" {
        return 0;
    }
    if c == " \n" {
        return PAGELEN;
    }
    if c == "\n" {
        return 1;
    }
    return 0;
}

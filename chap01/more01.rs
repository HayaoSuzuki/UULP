use std::io::BufRead;

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
            let mut file = std::fs::File::open(arg)?;
            let mut buffered_reader = std::io::BufReader::new(file);
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
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).ok().expect("Failed");
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

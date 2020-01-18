use metalabhackathon::run;

pub fn main() -> (){
    let mut stdout = std::io::stdout();

    match run(&mut stdout) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}


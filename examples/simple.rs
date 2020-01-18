use metalabhackathon::run;
use metalabhackathon::map::Level;

pub fn main() -> (){
    let mut stdout = std::io::stdout();

    let mut level = Level::new(16,16);
    match run(&mut stdout,&mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}


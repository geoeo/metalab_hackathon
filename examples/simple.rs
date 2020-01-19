use metalabhackathon::run;
use metalabhackathon::map::Grid;

pub fn main() -> (){
    let mut stdout = std::io::stdout();

    let mut level = Grid::new(16, 16);
    match run(&mut stdout,&mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}


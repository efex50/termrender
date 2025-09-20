
use std::env::args;

use crossterm::style::Color;
use termrender::{game::Game, TermPrint, RGB};


fn main() -> std::io::Result<()> {

    let arg = args().map(|a| a).collect::<Vec<String>>();
    let num = {
        match arg.get(1){
            Some(a) => a,
            None => panic!("{}",TermPrint{background:RGB::from(Color::AnsiValue(1)),foreground:RGB::from(Color::Red),text:"arg num1 not passed".to_string()}),
        }
    };
    let tick = {
        match i32::from_str_radix(num, 10){
            Ok(o) => o,
            Err(_) => panic!("the arg numero 1 is not an numero: the tick no"),
        }
    };
    
    let mut _main = Game::new("zort".to_string(),31.);

    _main.setup()?;
    
    main_loop(&mut _main, tick)?;
    
    _main.exit()?;
    
    return Ok(());
}


fn main_loop(m:&mut Game,t:i32) -> std::io::Result<()>{
    loop {
        
        m.poll_keys()?;
        
        if m.should_render(){ 
            
            let r = m.tick(t)?;
            if !r {break};
            
        }        
    }
    Ok(())

}




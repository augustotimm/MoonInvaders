extern crate classes;
use classes::game_world::GameWorld;
use classes::game_world::GameWorldT;
use classes::game_world::game_screen::GameScreenT;

fn main (){
    println!("{}", 4/3);

    let mut gw: GameWorld =GameWorld::new( 5,20);
    gw.update_mscreen();
    let mut screen = gw.get_screen();
    for line in screen{
        let str_scr:String = line.into_iter().collect();
        println!("{}",str_scr);
    }
}
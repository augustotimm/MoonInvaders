mod game_object;
use self::game_object::GameObjectT;

    




pub struct GameScreen{
    screen:Vec<Vec<char>>,
    down_limit:i8,
    right_limit:i8,
}
pub struct GameWorld{
    objects:Vec<game_object::GameObjectClass>,
    gs: GameScreen,
}
fn something(){
    let a = game_object::Base::new((0,0));
    
}
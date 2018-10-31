mod game_object;
pub enum GameImages{
    Alien,
    Player,
    Shot,
    Blank,
    SpaceGuide
}
    


impl GameImagesT for GameImages{
    fn value(&self)-> char{
        match *self{
            GameImages::Alien=> 'ゴ',
            GameImages::Player=>'A',
            GameImages::Shot => '!',
            GameImages::Blank=> ' ',
            GameImages::SpaceGuide=> '-',
        }
    }
}
trait  GameImagesT{
    fn value(&self)->char;
    
}

pub struct GameScreen{
    screen:Vec<Vec<char>>,
    down_limit:i8,
    right_limit:i8,
}
pub struct GameWorld{
    objects:Vec<game_object::GameObjectClass>,
    gs: GameScreen,
}
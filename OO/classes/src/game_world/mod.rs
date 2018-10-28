
pub enum GameImages{
    Alien,
    Player,
    Shot,
    Blank,
    SpaceGuide
}
    
impl GameImages{
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


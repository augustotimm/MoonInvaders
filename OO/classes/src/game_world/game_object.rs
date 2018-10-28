struct Base{
    position: (i32,i32),
    img:char,
}

impl GameObjectT<Base> for Base{
    
    fn new(_position:(i32,i32))->Base{
        Base{
            position : _position,
            img: ''
        }                        
    }
}

impl Base{
    pub fn get_position(&self)->(i32,i32){
        return self.position;
    }
    pub fn get_img(&self)->char{
        return self.img;
    }
}

pub struct Alien{
    my_base:Base,
    

}

pub struct Player{
    my_base:Base,

}


impl AlienT for Alien{
    fn move_alien(&mut self,speed:i32)->(i32,i32){
        let old_pos = self.my_base.position;
        
        return (1,2);
    }
}

impl GameObjectT<Alien> for Alien{
    fn new(_position:(i32,i32))-> Alien{
        Alien{
            my_base: Base::new(_position),
        }
    }
}

pub trait AlienT : GameObjectT<Alien>{
    fn move_alien(&mut self, speed:i32)->(i32,i32);
}
pub trait GameObjectT<T>{

    fn new(_position:(i32,i32))->T;
}

pub enum GameObjectClass<Alien,Player,Base>{
    Alien(Alien),
    Player(Player),
    Base(Base),
}

impl GameObjectT<GameObjectClass<Alien,Player,Base>> for GameObjectClass<Alien,Player,Base>{
    fn new(_position:(i32,i32))->GameObjectClass<Alien,Player,Base>{
        GameObjectClass::Base(Base::new(_position))
    }
}


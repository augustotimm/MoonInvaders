
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

pub struct Base{
    position: (i8,i8),
    img:char,
}

impl GameObjectT<Base> for Base{
    
    fn new( _position:(i8,i8))->Base{
        Base{
            position : _position,
            img: ''
        }                        
    }
    fn new_wbase(_base:Base) ->Base{
        _base
    }
}

impl BaseT for Base{
    fn get_position(&self)->(i8,i8){
        return self.position;
    }

    fn get_img(&self)->char{
        return self.img;
    }
    fn set_pos(& mut self,npos:(i8,i8)){
        self.position = npos;
    }
}

pub trait BaseT{
    fn get_img(&self)->char;
    fn get_position(&self)->(i8,i8);
    fn set_pos(& mut self,npos:(i8,i8));
}

pub struct Alien{
    my_base:Base,
    

}


pub struct Player{
    my_base:Base,

}
impl PlayerT for Player{
    fn walk(& mut self,dir:bool,right_limit:i8)->errors::ScreenLimit{
        let old_pos = self.my_base.position;
        if dir{
            if old_pos.1 +1 > right_limit{
                return errors::ScreenLimit::Err;
            }
            else{
                self.my_base.set_pos((old_pos.0,old_pos.1+1));
                return errors::ScreenLimit::Ok(self.my_base.position);
            }
        }
        else{
            if old_pos.1 -1 < 0{
                return errors::ScreenLimit::Err;
            }
            else{
                self.my_base.set_pos((old_pos.0,old_pos.1-1));
                return errors::ScreenLimit::Ok(self.my_base.position);
            }
        }

    }

    fn shoot(&mut self)->Base{
        let mut pos  = self.my_base.position;
        pos.0 = pos.0 +1;
        let mut shoot:Base = Base::new_img(GameImages::Shot.value(),pos);
        return shoot;

    }
}
pub trait PlayerT{
    fn walk(& mut self,dir:bool,right_limit:i8)->errors::ScreenLimit;
    fn shoot(& mut self)->Base;
}
impl GameObjectT<Player> for Player{
    fn new(_position:(i8,i8))->Player{
        Player{
            my_base: Base::new_img(GameImages::Player.value(),_position),
        }
       
    }
    fn new_wbase(_base:Base) -> Player{
        Player{
            my_base: _base,
        }
    }
}

impl AlienT for Alien{
    fn move_alien(&mut self,speed:i8,down_limit:i8, right_limit:i8,dir:Direction)->errors::ScreenLimit{
        
        let old_pos = self.my_base.position;
        match dir{
            Direction::Up=>{}
            Direction::Down=>{
                if old_pos.0 + speed > down_limit{
                    return errors::ScreenLimit::Err;        
                }
                else{
                    self.my_base.position = (old_pos.0+speed,old_pos.1);
                    return errors::ScreenLimit::Ok(self.my_base.position);
                }
            }
            Direction::Left=>{
                if old_pos.1 - speed < 0{
                    return errors::ScreenLimit::Err;        
                }
                else{
                    self.my_base.position = (old_pos.0,old_pos.1-speed);
                    return errors::ScreenLimit::Ok(self.my_base.position);
                }
            }
            Direction::Right=>{
                if old_pos.1 + speed > right_limit{
                    return errors::ScreenLimit::Err;        
                }
                else{
                    self.my_base.position = (old_pos.0,old_pos.1+speed);
                    return errors::ScreenLimit::Ok(self.my_base.position);
                }
            }
        }
        errors::ScreenLimit::Err
        //return (1,2);
    }
}

impl GameObjectT<Alien> for Alien{
    fn new(_position:(i8,i8))->Alien{
        Alien{
            my_base: Base::new_img(GameImages::Alien.value(),_position),
        }
       
    }
    fn new_wbase(_base:Base) -> Alien{
        Alien{
            my_base: _base,
        }
    }

}

pub trait AlienT : GameObjectT<Alien>{
    fn move_alien(&mut self,speed:i8,down_limit:i8, right_limit:i8, dir:Direction)->errors::ScreenLimit;
}
pub trait GameObjectT<T>{

    fn new(_position:(i8,i8))->T;
    fn new_img<A:GameObjectT<A>>(_img:char,_position:(i8,i8))->A{
        let nb = Base{
            position : _position,
            img: _img,
        };

        let na:A = A::new_wbase(nb);
        
        return na;
        
    }

    fn new_wbase(_base:Base) ->T;

}

pub enum GameObjectClass{
    Alien(Alien),
    Player(Player),
    Base(Base),
}

pub mod errors{
    pub enum ScreenLimit{
        Err,
        Ok((i8,i8)),
    }
}

pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

#[test]
fn test_mv_alien_r(){
    let mut al = Alien::new((0,0));
    let res = al.move_alien(1, 2, 2, Direction::Right);
    match res{
        errors::ScreenLimit::Err=>{
           // assert!(false,"Off limits");
        },
        errors::ScreenLimit::Ok((0,1))=>{
            assert!(true,"On limits");
        },
        _=>{
            //assert!(false,"UNKNOWN");
        }
    }
}
#[test]
fn test_mv_alien_l(){
    let mut al = Alien::new((0,0));
    let res = al.move_alien(1, 2, 2, Direction::Left);
    match res{
        errors::ScreenLimit::Err=>{
           assert!(true,"Off limits");
        },
        errors::ScreenLimit::Ok((0,1))=>{
            panic!("ANDOU DIREITA")
        },
        _=>{
            panic!("UNKNOWN");
        }
    }
    let mut al = Alien::new((0,1));
    let res = al.move_alien(1, 2, 2, Direction::Left);
    match res{
        errors::ScreenLimit::Err=>{
           panic!("Off limits");
        },
        errors::ScreenLimit::Ok((0,0))=>{
            assert!(true,"ANDOU ESQUERDA")
        },
        _=>{
            panic!("UNKNOWN");
        }
    }
}

#[test]
fn test_mv_alien_d(){
    let mut al = Alien::new((0,0));
    let res = al.move_alien(1, 2, 2, Direction::Down);
    match res{
        errors::ScreenLimit::Err=>{
           panic!("Off limits");
        },
        errors::ScreenLimit::Ok((1,0))=>{
            assert!(true,"ANDOU BAIXO")
        },
        _=>{
            panic!("UNKNOWN");
        }
    }
}
#[test]
fn test_mv_alien_u(){
    let mut al = Alien::new((0,0));
    let res = al.move_alien(1, 2, 2, Direction::Up);
    match res{
        errors::ScreenLimit::Err=>{
           assert!(true,"Off limits");
        },
        errors::ScreenLimit::Ok((1,0))=>{
            panic!("ANDOU BAIXO");
        },
        _=>{
            panic!("UNKNOWN");
        }
    }
}

#[test]
fn test_walk_r(){
    let mut pl  = Player::new((1,1));
    
    pl.walk(true, 2);
    assert_eq!(pl.my_base.position,(1,2));
    let res = pl.walk(true, 2);
    match res{
        errors::ScreenLimit::Err=>{
            assert!(true,"ERRO ESPERADO");
        }
        _=>{panic!();}
    }

}

#[test]
fn test_walk_l(){
    let mut pl  = Player::new((1,1));
    
    pl.walk(false, 5);
    assert_eq!(pl.my_base.position,(1,0));
    let res = pl.walk(false, 2);
    match res{
        errors::ScreenLimit::Err=>{
            assert!(true,"ERRO ESPERADO");
        }
        _=>{panic!();}
    }

}

#[test]
fn test_shoot(){
    let mut pl  = Player::new((0,1));
    let mut shot = pl.shoot();
    assert_eq!(shot.position,(1,1));
}
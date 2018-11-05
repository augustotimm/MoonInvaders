mod game_object;
use self::game_object::GameObjectT;
use self::game_object::Base;

use self::game_object::GameImagesT;
use self::game_object::AlienT;
use self::game_object::PlayerT;
use self::game_object::BaseT;
use self::game_object::GameObjectClass;
use self::game_object::Alien;
use self::game_object::Player;
use self::game_object::GameImages;
use self::game_screen::GameScreen;
use self::game_screen::GameScreenT;
use self::game_object::ShotT;
use self::game_object::Shot;
use self::game_object::GameObjectClassT;
mod game_screen;



#[derive(Copy,Clone,PartialEq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

pub mod errors{
    use super::game_object::GameObjectClass;
    #[derive(Copy,Clone)]
    pub enum ScreenLimit{
        Err,
        Ok((i8,i8)),
    }
    pub enum CollisionErr{
        Err,
        Ok(GameObjectClass),
    }
}












pub struct GameWorld{
    objects:Vec<game_object::GameObjectClass>,
    gs: GameScreen,
    cur_dir:Direction,
    speed:i8,
    points:u32,
    r_key:char,
    l_key:char,
    s_key:char,
}

impl GameWorld{
    fn move_all(&mut self, delegate_collision:fn(GameObjectClass,GameObjectClass, &u32)-> errors::CollisionErr)->errors::ScreenLimit{
        let limits = self.gs.get_limits();
        let nobject: GameObjectClass = GameObjectClass::Base( game_object::Base::new((0i8,0i8)) );
        let mut new_screen:Vec<Vec<GameObjectClass>> = vec![vec![nobject;(limits.1 +1) as usize];(limits.0 +1) as usize];
        let objs =self.objects.clone();
        let mut new_objs:Vec<GameObjectClass> = Vec::new();
        for x in objs{
            match x{
                GameObjectClass::Player(p)=>{
                    let al_old_pos = p.get_position();
                    let old = new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize];
                    match old{                        
                        GameObjectClass::Base(b)=>{
                            new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize] = GameObjectClass::Player(p);
                            continue;
                        },
                        _=>{
                            self.end_game(false);
                        }
                        
                    }
                    
                },
                GameObjectClass::Alien(mut a)=>{
                    let mv_res =a.move_alien(self.speed,limits.0,limits.1,self.cur_dir);
                    match mv_res{
                        errors::ScreenLimit::Err=>{
                            return errors::ScreenLimit::Err;
                            
                        },
                        errors::ScreenLimit::Ok(o)=>{
                            let al_n_pos = a.get_position();
                            let old = new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize];
                            match old{
                                GameObjectClass::Base(b)=>{
                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = GameObjectClass::Alien( a);
                                    continue;

                                },
                                GameObjectClass::Alien(a)=>{},
                                GameObjectClass::Player(p)=>{
                                    self.end_game(false);
                                },
                                GameObjectClass::Shot(s)=>{
                                    let res = delegate_collision(GameObjectClass::Alien(a),GameObjectClass::Shot(s), &self.points);
                                    match res{
                                        errors::CollisionErr::Err=>{

                                        },
                                        errors::CollisionErr::Ok(o)=>{
                                            match o{
                                                GameObjectClass::Base(b)=>{
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nobject;
                                                    continue;
                                                }
                                                x_=>{       
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = x;
                                                    
                                                }

                                            }
                                            
                                        }
                                    }
                                    
                                },
                            }

                        }

                    }
                    
                },
                GameObjectClass::Shot(mut s)=>{
                    let old_pos = s.get_position();
                    let res = s.move_shot(limits.0,limits.1);
                    match res{
                        errors::ScreenLimit::Err=>{
                            new_screen[old_pos.0 as usize][old_pos.1 as usize] = nobject;
                            continue;
                        }
                        errors::ScreenLimit::Ok(o)=>{
                            let s_n_pos = s.get_position();
                            let old = new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize];
                            match old{
                                GameObjectClass::Base(b)=>{
                                    new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = GameObjectClass::Shot(s);
                                    continue;

                                },
                                GameObjectClass::Alien(a)=>{
                                    let res = delegate_collision(GameObjectClass::Alien(a),GameObjectClass::Shot(s), &self.points);
                                    match res{
                                        errors::CollisionErr::Err=>{

                                        },
                                        errors::CollisionErr::Ok(o)=>{
                                            match o{
                                                GameObjectClass::Base(b)=>{
                                                    new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = nobject;
                                                    continue;
                                                }
                                                x_=>{       
                                                    new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = x;
                                                    
                                                }

                                            }
                                            
                                        }
                                    }
                                },
                                GameObjectClass::Player(p)=>{
                                    self.end_game(false);
                                },
                                GameObjectClass::Shot(ss)=>{
                                    
                                    
                                },
                            }
                        }
                    }

                },
                GameObjectClass::Base(b)=>{},
            }
        }
        let mut new_objs:Vec<GameObjectClass> = Vec::new();
        for line in new_screen{
            for element in line{
                new_objs.push(element);
            }
        }
        self.objects = new_objs;
        errors::ScreenLimit::Ok((0,0))
    }
    fn get_obj(&self)->Vec<GameObjectClass>{
        return self.objects.clone();
    }
}

impl GameWorldT<GameWorld> for GameWorld{
    fn move_world(&mut self, delegate_collision:fn(GameObjectClass,GameObjectClass,&u32)-> errors::CollisionErr){
        let old_dir = self.cur_dir;
        let mut new_dir:Direction = Direction::Up;
        
        let ret = self.move_all(delegate_collision);
        match ret{
            errors::ScreenLimit::Err=>{
                
                match old_dir{
                    Direction::Down=>{},
                    Direction::Up=>{},
                    _=>{
                        self.cur_dir = Direction::Down;
                    }
                }
                let ret2 = self.move_all(delegate_collision);
                match ret2{
                    errors::ScreenLimit::Ok(o)=>{
                        new_dir = self.cur_dir;
                    },  
                    _=>{},      
                }

            },
            errors::ScreenLimit::Ok(o)=>{
                new_dir = self.cur_dir;
            },
        }
        
        if (old_dir != new_dir) && (new_dir != Direction::Up ){
           self.cur_dir = if old_dir == Direction::Right{Direction::Left }else{Direction::Right};
        }
        
    }
    fn new(d_limit: i8, l_limit: i8)->GameWorld{
        GameWorld{
            objects:Vec::new(),
            gs: GameScreen::new(d_limit, l_limit),
            cur_dir:Direction::Right,
            speed:1,
            points:0u32,
            r_key:'d',
            l_key:'a',
            s_key:'k',
        }
    }
    
    fn set_screen(&mut self,gs:GameScreen){
        self.gs = gs;
    }

    //Override
    fn new_w_screen(_gs:GameScreen)->GameWorld{
        let limits:(i8,i8) = _gs.get_limits();
        GameWorld{
            objects:Vec::new(),
//            player_:Player::new((limits.0,(limits.1/2 as i8))),
            gs: _gs,
            cur_dir:Direction::Right,
            speed:1,
            points:0u32,
            r_key:'d',
            l_key:'a',
            s_key:'k',
        }
    }
    fn end_game(&self,win:bool){

    }
    fn add_object(&mut self, go:GameObjectClass){
        self.objects.push(go)
    }

    fn enter_input(&mut self,key:char){
        let mut shoot = false;
        let mut dir:bool = true;
        let limit = self.gs.get_limits().1;
        let right = self.r_key;
        let left = self.l_key;
        let shoot_key = self.s_key;
        match key{
            k if key == right =>{
                dir = true;
            },
            k if key == left=>{
                dir = false;
            },
            k if key == shoot_key=>{
                shoot = true;
            },
            _=>{},
        }
        
        let len = self.objects.len();
        for i in 0..len{
            let mut  obj = self.objects[i];
            match obj{
                GameObjectClass::Player(mut p)=>{
                    if shoot{
                        
                        let new_shot=p.shoot();
                        self.objects.remove(i);
                        self.objects.insert(i, GameObjectClass::Player(p));
                        self.objects.push(GameObjectClass::Shot(new_shot));

                    }
                    else{
                        p.walk(dir,limit);
                        self.objects.remove(i);
                        self.objects.insert(i, GameObjectClass::Player(p));
                    }
                },
                _=>{},
            }
        }        
                       
            
        

    }
}

pub trait GameWorldT<G:GameWorldT<G>>{
    fn move_world(&mut self,delegate_collision:fn(GameObjectClass,GameObjectClass,&u32)-> errors::CollisionErr);
    fn new(d_limit: i8, l_limit: i8)->G;
    fn set_screen(&mut self,gs:GameScreen);
    fn new_w_screen(gs:GameScreen)->G{
        let limits:(i8,i8) = gs.get_limits();
        let mut new_world:G = G::new(limits.0,limits.1);
        new_world.set_screen(gs);
        new_world
    }
    fn end_game(&self,win:bool);
    fn add_object(&mut self, go:GameObjectClass);
    fn enter_input(&mut self,key:char);
}

#[test]
fn test_movew(){
    let mut sc = GameScreen::new(3i8, 2i8);
       
    let al_pos = (0i8,0i8);
    let pl_pos = (2i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::new_player(pl);
    gw.add_object(GameObjectClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) );
    gw.add_object(gpl);
    gw.move_world(tes_delegate_collision);
    let objects = gw.get_obj();
    for obj in objects{
        match obj{
            GameObjectClass::Alien(a)=>{
                assert_eq!(a.get_position(),(0,1));
            }
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),pl_pos);
            }
            _=>{}
        }
    }
}


#[test]
fn test_movew_limit(){
    let mut sc = GameScreen::new(3i8, 2i8);
       
    let al_pos = (0i8,1i8);
    let pl_pos = (2i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::new_player(pl);
    gw.add_object(GameObjectClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) );
    gw.add_object(gpl);
    gw.move_world(tes_delegate_collision);
    let objects = gw.get_obj();
    for obj in objects{
        match obj{
            GameObjectClass::Alien(a)=>{
                assert_eq!(a.get_position(),(1,1));
            }
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),pl_pos);
            }
            _=>{}
        }
    }
    gw.move_world(tes_delegate_collision);
    let objects = gw.get_obj();
    for obj in objects{
        match obj{
            GameObjectClass::Alien(a)=>{
                assert_eq!(a.get_position(),(1,0));
            }
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),pl_pos);
            }
            _=>{}
        }
    }
}
fn tes_delegate_collision(go1: GameObjectClass,go2:GameObjectClass,points:&u32)-> errors::CollisionErr{
    errors::CollisionErr::Err
}

#[test]
fn test_input(){
    let mut sc = GameScreen::new(3i8, 2i8);
       
    let al_pos = (0i8,1i8);
    let pl_pos = (2i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::new_player(pl);
    gw.add_object(GameObjectClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) );
    gw.add_object(gpl);
    gw.enter_input('a');
    for obj in gw.get_obj(){
        match obj{
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),(2,0));
            }
            _=>{}
        }
    }
    gw.enter_input('d');
    for obj in gw.get_obj(){
        match obj{
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),(2,1));
            }
            _=>{}
        }
    }
    gw.enter_input('k');
    for obj in gw.get_obj(){
        match obj{
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),(2,1));
            }
            GameObjectClass::Shot(s)=>{
                assert_eq!(s.get_position(),(1,1));
            }
            _=>{}
        }
    }
}
/*
fn something(){
    let a = game_object::Base::new((0,0));
    a.get_img();
    
}
*/


/*
***** MOVE
let res = a.move_alien(1i8,self.right_limit,self.down_limit,self.cur_dir );
                        match res{
                            errors::ScreenLimit::Err=>{
                                ok_dir = false;
                                match self.cur_dir{
                                    Direction::Right=>{
                                        new_dir = Direction::Down;
                                        self.cur_dir = Direction::Left;
                                    },
                                    Direction::Left=>{
                                        new_dir = Direction::Down;
                                        self.cur_dir = Direction::Right;
                                    },
                                    _=>{
                                        //Throw Error
                                    }
                                }
                            }
                            errors::ScreenLimit::Ok(x)=>{
                                new_screen[x.0 as usize][x.1 as usize] = a.get_img();
                            }
                        }
*/
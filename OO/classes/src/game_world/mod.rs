pub mod game_object;
pub mod game_screen;


use self::game_object::SupAlien;
use self::game_object::GameObjectT;
use self::game_object::Base;
use self::game_object::AliensClass;
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
use self::game_object::SupAlienT;
use self::game_object::Shot;


//use self::game_object::GameObjectClassT;


extern crate rand;

use game_world::rand::prelude::*;



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
        Die,
    }
}









pub struct GameWorld{
    objects:Vec<game_object::GameObjectClass>,
    gs: GameScreen,
    cur_dir:Direction,
    speed:i8,
    points:i32,
    r_key:char,
    l_key:char,
    s_key:char,
    end:bool,
    extra_lifes:i32,
    shot_limit:i8,
}

impl GameWorld{
    fn move_all(&mut self, delegate_collision:fn(GameObjectClass,GameObjectClass, &mut i32)-> errors::CollisionErr)->errors::ScreenLimit{
        let limits = self.gs.get_limits();
        let nobject: GameObjectClass = GameObjectClass::Base( game_object::Base::new((0i8,0i8)) );
        let mut new_screen:Vec<Vec<GameObjectClass>> = vec![vec![nobject;(limits.1) as usize];(limits.0) as usize];
        let objs =self.objects.clone();
        let objs2 =self.objects.clone();
        
        
        for x in objs{
            
            match x{
                GameObjectClass::Player(p)=>{
                    let al_old_pos = p.get_position();
                    let old = new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize];
                    let res = delegate_collision(GameObjectClass::Player(p),old,&mut self.points);
                    
                    match res{
                        errors::CollisionErr::Err=>{
                            new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize] = GameObjectClass::Player(p);
                        },
                        errors::CollisionErr::Ok(o)=>{
                            new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize] = GameObjectClass::Player(p);
                            continue;
                        },
                        errors::CollisionErr::Die=>{
                            let new_p =self.damage();
                            let lf = new_p.get_lifes();
                            if lf <=0{
                                self.end_game(false);
                                return errors::ScreenLimit::Ok((-1,-1));
                            }
                            else{
                                new_screen[al_old_pos.0 as usize][al_old_pos.1 as usize] = GameObjectClass::Player(new_p);
                            }
                        }
                    }
                    
                    
                },
                GameObjectClass::Alien(mut al)=>{
                    match al{
                        AliensClass::Alien(mut a)=>{
                            let mv_res =a.move_alien(self.speed,limits.0,limits.1,self.cur_dir);
                            match mv_res{
                                errors::ScreenLimit::Err=>{
                                    return errors::ScreenLimit::Err;        
                                },
                                errors::ScreenLimit::Ok(o)=>{
                                    let al_n_pos = a.get_position();
                                    let old = new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize];

                                    let res = delegate_collision( GameObjectClass::Alien(AliensClass::Alien(a)),old,&mut self.points);

                                    match res{
                                        errors::CollisionErr::Err=>{

                                        },
                                        errors::CollisionErr::Ok(o)=>{
                                            match o{
                                                GameObjectClass::Base(b)=>{
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nobject;
                                                    continue;
                                                }
                                                nx=>{       
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nx;
                                                    
                                                }

                                            }
                                            
                                        },
                                        errors::CollisionErr::Die=>{
                                            new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nobject;
                                            continue;
                                        },
                                    }   
                

                                },
                        


                            }

                        },
                        AliensClass::SupAlien(mut a)=>{
                           let mv_res =a.move_alien(self.speed,limits.0,limits.1,self.cur_dir);
                           
                            match mv_res{
                                errors::ScreenLimit::Err=>{
                                    return errors::ScreenLimit::Err;        
                                },
                                errors::ScreenLimit::Ok(o)=>{
                                    let al_n_pos = a.get_position();
                                    let old = new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize];
                                    let res = delegate_collision( GameObjectClass::Alien(AliensClass::SupAlien(a)),old,&mut self.points);

                                    match res{
                                        errors::CollisionErr::Err=>{

                                        },
                                        errors::CollisionErr::Ok(o)=>{
                                            match o{
                                                GameObjectClass::Base(b)=>{
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nobject;
                                                    continue;
                                                },
                                                GameObjectClass::Alien(AliensClass::SupAlien(mut sa))=>{
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = GameObjectClass::Alien(AliensClass::SupAlien(sa));
                                                    let y = random::<f32>();
                                                    if y >= 0.8f32{
                                                    let new_shot =sa.shoot();
                                                    let shot_pos = new_shot.get_position();
                                                    let old_obj = new_screen[shot_pos.0 as usize][shot_pos.1  as usize];
                                                    //let shot_res = self.sup_alien_shoot(new_shot, old_obj, delegate_collision);

                                                    let shot_col = delegate_collision(GameObjectClass::Shot(new_shot),old_obj,&mut self.points);
                                                    match shot_col{
                                                        errors::CollisionErr::Err=>{},
                                                        errors::CollisionErr::Ok(o)=>{
                                                            new_screen[shot_pos.0 as usize][shot_pos.1 as usize] = o;
                                                        },
                                                        errors::CollisionErr::Die=>{
                                                            let new_p =self.damage();
                                                            let lf = new_p.get_lifes();
                                                            if lf <=0{
                                                                self.end_game(false);
                                                                return errors::ScreenLimit::Ok((-1,-1));
                                                            }
                                                            else{
                                                                new_screen[shot_pos.0 as usize][shot_pos.1 as usize] = GameObjectClass::Player(new_p);
                                                            }
                                                        }
                                                        
                                                    }
                                                        
                                                
                                            }
                                            continue;
                                                }
                                                nx=>{       
                                                    new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nx;
                                                    
                                                }

                                            }
                                            
                                        },
                                        errors::CollisionErr::Die=>{
                                            new_screen[al_n_pos.0 as usize][al_n_pos.1 as usize] = nobject;
                                            continue;
                                        },
                                    }  
                                   

                                },
                        


                            }
                            
                        },

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
                           
                            let res = delegate_collision(GameObjectClass::Shot(s),old,&mut self.points);

                            match res{
                                errors::CollisionErr::Err=>{},
                                errors::CollisionErr::Ok(o)=>{
                                    let opos = o.get_position().0;
                                    if opos != limits.0{
                                        new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = GameObjectClass::Shot(s);
                                    }                                    
                                    continue;

                                }
                                errors::CollisionErr::Die=>{
                                    if s.dir == Direction::Down{
                                        let new_p =self.damage();
                                        let lf = new_p.get_lifes();
                                        if lf <=0{
                                            self.end_game(false);
                                            return errors::ScreenLimit::Ok((-1,-1));
                                        }
                                        else{
                                            new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = GameObjectClass::Player(new_p);
                                        }
                                    }
                                    new_screen[s_n_pos.0 as usize][s_n_pos.1 as usize] = nobject;
                                    continue;
                                },
                            }                        
                        }
                    }

                },
                GameObjectClass::Base(b)=>{
                    
                },
            }
        }
        let mut new_objs:Vec<GameObjectClass> = Vec::new();
        let mut win: bool = true;
        for line in new_screen{
            for element in line{
                match element{
                    GameObjectClass::Base(b)=>{
                        if b.get_img() != GameImages::None.value(){
                            new_objs.push(GameObjectClass::Base(b));
                        }
                    },
                    GameObjectClass::Alien(a)=>{
                        if a.get_position().0 >= self.gs.get_limits().0-1{
                            self.end_game(false);
                            return errors::ScreenLimit::Ok((-1,-1));

                            
                        }
                        win = false;
                        new_objs.push(GameObjectClass::Alien(a));
                    },
                    GameObjectClass::Shot(s)=>{
                        if s.get_position().0 >= self.gs.get_limits().0-1{

                        }
                        else{
                            new_objs.push(GameObjectClass::Shot(s));
                        }
                    },
                    x=>{
                        new_objs.push(x);
                    },
                }
                
            }
        }
        self.objects = new_objs;
        if win{
            self.end_game(true);
        }
        
        errors::ScreenLimit::Ok((0,0))
    }

  
    fn get_obj(&self)->Vec<GameObjectClass>{
        return self.objects.clone();
    }
    fn generate_objects(&mut self){
        let  limits = self.gs.get_limits();
        let start_screen = (limits.0/3);
        let al_limits:(i8,i8)= (limits.0 - (start_screen +1), (limits.1 - 3)); //Remove 1 terço da tela para movimentacao dos aliens e mais uma linha para o jogador
        

        for i in 0..al_limits.0{
            for j in 0..al_limits.1{
                if j%2 ==0{
                    
                    if i < start_screen{
                        self.generate_sup_alien((i,j));
                    }
                    else{
                        self.generate_alien((i,j));                    
                    }
                }
                
            }
        }
        self.generate_player((limits.0-1,limits.1/2));

        /*for i in 0..limits.1{
            if i != limits.1/2{
                self.generate_guide((limits.0-1,i));
            }
            else{
                
            }
        }*/
    }

    fn generate_alien(&mut self, pos:(i8,i8) ){
        let al =Alien::new(pos);
          let gal=GameObjectClass::Alien(  AliensClass::Alien(al) );
          self.objects.push(gal);
    }
    fn generate_sup_alien(&mut self,pos:(i8,i8)){
        let al = SupAlien::new( pos);
        let gal =GameObjectClass::Alien(  AliensClass::SupAlien(al) );
        self.objects.push(gal);
    }
    fn generate_guide(&mut self,pos:(i8,i8)){
        let gd = Base::new_img(GameImages::SpaceGuide.value(), pos);
        let ggd =GameObjectClass::Base( gd );
        self.objects.push(ggd);
    }
    fn generate_player(&mut self,pos:(i8,i8)){
        let pl = Player::new_img(GameImages::Player.value(), pos);
        let gpl =GameObjectClass::Player( pl );
        self.objects.push(gpl);
    }
    fn reset(&mut self){
        self.objects = Vec::new();
        self.generate_objects();
        return;
    }
}

impl GameWorldT<GameWorld> for GameWorld{
    fn move_world(&mut self, delegate_collision:fn(GameObjectClass,GameObjectClass,&mut i32)-> errors::CollisionErr){
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
                        if o.0 != -1i8{
                            new_dir = self.cur_dir;
                        }
                    },  
                    _=>{},      
                }

            },
            errors::ScreenLimit::Ok(o)=>{
                if o.0 != -1i8{
                    new_dir = self.cur_dir;
                }
                
            },
        }
        
        if (old_dir != new_dir) && (new_dir != Direction::Up ){
           self.cur_dir = if old_dir == Direction::Right{Direction::Left }else{Direction::Right};
        }
        errors::CollisionErr::Ok( GameObjectClass::Base(Base::new_img(GameImages::None.value(),(0,0) )));
    }
    fn new(d_limit: i8, l_limit: i8)->GameWorld{
        let mut gw =GameWorld{
            objects:Vec::new(),
            gs: GameScreen::new(d_limit, l_limit),
            cur_dir:Direction::Right,
            speed:1,
            points:0i32,
            r_key:'d',
            l_key:'a',
            s_key:'k',
            end:false,
            extra_lifes:0,
            shot_limit:5,
        };
        gw.generate_objects();
        gw
    }
    
    fn set_screen(&mut self,gs:GameScreen){
        self.gs = gs;
    }

    //Override
    fn new_w_screen(_gs:GameScreen)->GameWorld{
        let limits:(i8,i8) = _gs.get_limits();
        let mut gw =GameWorld{
            objects:Vec::new(),

            gs: _gs,
            cur_dir:Direction::Right,
            speed:1,
            points:0i32,
            r_key:'d',
            l_key:'a',
            s_key:'k',
            end:false,
            extra_lifes:0,
            shot_limit:5,
        };
        gw.generate_objects();
        gw
    }
    fn end_game(&mut self,win:bool){
        if win{
            self.reset();
            return;
        }
        self.gs.end_screen(win,self.points);
        self.end = true;
        self.objects = Vec::new();
        return;
    }
    fn add_object(&mut self, go:GameObjectClass){
        self.objects.push(go)
    }

    fn enter_input(&mut self,key:char,delegate_collision:fn(GameObjectClass,GameObjectClass,&mut i32)-> errors::CollisionErr){
        let mut shoot = false;
        let mut dir:bool = true;
        let limit = self.gs.get_limits().1;
        let right = self.r_key;
        let left = self.l_key;
        let shoot_key = self.s_key;
        match key{
            kr if kr == right =>{
                dir = true;
            },
            kl if kl == left=>{
                dir = false;
            },
            ks if ks == shoot_key=>{
                let mut count = 0;
                for obj in self.objects.clone(){
                    match obj{
                        GameObjectClass::Shot(s)=>{
                            if s.dir == Direction::Up{
                                count = count +1;
                            }
                        },
                        _=>{},
                    }
                }
                if count < self.shot_limit{
                    shoot = true;
                } 
                else{
                    return;
                }              
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
                        //self.objects.remove(i);
                        //self.objects.insert(i, GameObjectClass::Player(p));
                        for j in 0..len{
                            let x = self.objects[j];
                            if x.get_position() == new_shot.get_position(){
                                let res = delegate_collision(GameObjectClass::Shot(new_shot),x,&mut self.points);
                                match res{
                                    errors::CollisionErr::Err=>{},
                                    errors::CollisionErr::Ok(o)=>{
                                        match o{
                                            GameObjectClass::Base(b)=>{
                                                self.objects.remove(j);
                                                self.objects.push(GameObjectClass::Shot(new_shot));
                                                return;
                                            },
                                            x=>{                     
                                                
                                                
                                            }
                                        }
                                    },
                                    errors::CollisionErr::Die=>{
                                        self.objects.remove(j);
                                        return;
                                    },
                                }
                                break;
                            }
                        }
                        self.objects.push(GameObjectClass::Shot(new_shot));                        

                    }
                    else{
                        let res = p.walk(dir,limit);
                        match res{
                            errors::ScreenLimit::Err=>{

                            },
                            errors::ScreenLimit::Ok(o)=>{
                                self.objects.remove(i);
                                self.objects.push(GameObjectClass::Player(p));
                            },
                        }
                        
                    }
                },
                _=>{},
            }
        }        
                       
            
        

    }
    fn get_end(&self)->bool{
        self.end
    }
    fn update_mscreen(&mut self){
        self.gs.update_screen(&self.objects);
    }
    fn get_screen(&self)->Vec<Vec<char>>{
        self.gs.get_screen()
    }

    fn get_points(&self)->i32{
        self.points
    }
    fn damage(&mut self)->Player{
        for i in 0..self.objects.len(){
            let obj = self.objects[i];
            match obj{
                GameObjectClass::Player(mut p)=>{
                    p.die();
                    self.objects.remove(i);
                    self.objects.insert(i, GameObjectClass::Player(p));
                    
                    return  p;
                },
                _=>{},
            }
        }
        return Player::new((0,0));
    }
    fn gain_life(&mut self)->bool{
        let extra = self.points/100 - self.extra_lifes;
        if extra >0{
            let mut pl:Player = Player::new((0,0));
            for j in 0..self.objects.len(){
                match self.objects[j]{
                    GameObjectClass::Player(p)=>{
                        pl = p;
                        self.objects.remove(j);
                    }
                    _=>{},
                }
            }
            for i in 0..extra{
                pl.gain_life();
                self.extra_lifes = self.extra_lifes +1;
            }
            self.objects.push(GameObjectClass::Player(pl));
            return true;
        }
        

        return false;
    }
    fn get_lifes(&self)->i32{
        for obj in self.objects.clone(){
            match obj{
                GameObjectClass::Player(p)=>{
                    return p.get_lifes();
                },
                _=>{},
            }
        }
        return 0;
    }

}

pub trait GameWorldT<G:GameWorldT<G>>{
    fn move_world(&mut self,delegate_collision:fn(GameObjectClass,GameObjectClass,&mut i32)-> errors::CollisionErr);
    fn new(d_limit: i8, l_limit: i8)->G;
    fn set_screen(&mut self,gs:GameScreen);
    fn new_w_screen(gs:GameScreen)->G{
        let limits:(i8,i8) = gs.get_limits();
        let mut new_world:G = G::new(limits.0,limits.1);
        new_world.set_screen(gs);
        new_world
    }
    fn end_game(&mut self,win:bool);
    fn add_object(&mut self, go:GameObjectClass);
    fn enter_input(&mut self,key:char,delegate_collision:fn(GameObjectClass,GameObjectClass,&mut i32)-> errors::CollisionErr);
    fn get_end(&self)->bool;
    fn update_mscreen(&mut self);
    fn get_screen(&self)->Vec<Vec<char>>;
    fn get_points(&self)->i32;
    fn damage(&mut self)->Player;
    fn gain_life(&mut self)->bool;
    fn get_lifes(&self)->i32;
}

#[test]
fn test_movew(){
    let mut sc = GameScreen::new(3i8, 2i8);
       
    let al_pos = (0i8,0i8);
    let pl_pos = (2i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::Player(pl);
    gw.add_object(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) ) );
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
    let gpl : GameObjectClass = GameObjectClass::Player(pl);
    gw.add_object(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) )  );
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
fn tes_delegate_collision(go1: GameObjectClass,go2:GameObjectClass,points:&mut i32)-> errors::CollisionErr{
    errors::CollisionErr::Err
}

#[test]
fn test_input(){
    let mut sc = GameScreen::new(3i8, 2i8);
       
    let al_pos = (0i8,1i8);
    let pl_pos = (2i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::Player(pl);
    gw.add_object(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) )  );
    gw.add_object(gpl);
    gw.enter_input('a',tes_delegate_collision);
    for obj in gw.get_obj(){
        match obj{
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),(2,0));
            }
            _=>{}
        }
    }
    gw.enter_input('d',tes_delegate_collision);
    gw.move_world(tes_delegate_collision);
    let mut pos:bool = false;
    for obj in gw.get_obj(){
        match obj{
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),(2,1));
                pos = true;
            }
            _=>{}
        }
    }
    assert_eq!(pos,true);
    gw.enter_input('k',tes_delegate_collision);
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


#[test]
fn test_movew_sup(){
    let mut sc = GameScreen::new(4i8, 2i8);
       
    let al_pos = (0i8,0i8);
    let sal_pos = (1,0);
    let pl_pos = (3i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::Player(pl);
    gw.add_object(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) ) );
    gw.add_object(GameObjectClass::Alien( AliensClass::SupAlien( SupAlien::new_img(GameImages::SAlien.value(), sal_pos ) ) ) );
    gw.add_object(gpl);
    gw.move_world(tes_delegate_collision);
    let objects = gw.get_obj();
    for obj in objects{
        match obj{
            GameObjectClass::Alien(a)=>{
                match a{
                    AliensClass::SupAlien(sal)=>{
                        assert_eq!(a.get_position(),(1,1));
                    },
                    AliensClass::Alien(al)=>{
                        assert_eq!(a.get_position(),(0,1));
                    },
                }
                
            }
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),pl_pos);
            }
            GameObjectClass::Shot(s)=>{
                assert_eq!(s.get_position(),(2,1));
            },
            _=>{},
        }
    }
}


#[test]
fn test_movew_lose(){
    let mut sc = GameScreen::new(4i8, 2i8);
       
    let al_pos = (0i8,0i8);
    let sal_pos = (1,0);
    let pl_pos = (3i8,1i8);
    let mut gw = GameWorld::new_w_screen(sc);
    let pl = Player::new_img(GameImages::Player.value(),pl_pos);
    let gpl : GameObjectClass = GameObjectClass::Player(pl);
    gw.add_object(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) ) );
    gw.add_object(GameObjectClass::Alien( AliensClass::SupAlien( SupAlien::new_img(GameImages::SAlien.value(), sal_pos ) ) ) );
    gw.add_object(gpl);
    gw.move_world(tes_delegate_collision);
    let objects = gw.get_obj();
    for obj in objects{
        match obj{
            GameObjectClass::Alien(a)=>{
                match a{
                    AliensClass::SupAlien(sal)=>{
                        assert_eq!(a.get_position(),(1,1));
                    },
                    AliensClass::Alien(al)=>{
                        assert_eq!(a.get_position(),(0,1));
                    },
                }
                
            }
            GameObjectClass::Player(p)=>{
                assert_eq!(p.get_position(),pl_pos);
            }
            GameObjectClass::Shot(s)=>{
                assert_eq!(s.get_position(),(2,1));
            },
            _=>{},
        }
    }
}

#[test]
fn test_bug_player_disappear(){
    let mut gw: GameWorld =GameWorld::new( 10,20);
    gw.update_mscreen();

    for i in 0..15{
        let data:char;
        if i%2 ==0{
            data = 'd';
        }
        else{
            data = 'a';
        }
        gw.enter_input(data,collision);
        gw.move_world(tes_delegate_collision);
    }
    let objs = gw.get_obj();
    let mut contain:bool = false;
    for obj in objs{
        match obj{
            GameObjectClass::Player(p)=>{
                contain = true;
            },
            _=>{}
        }
    }
    assert_eq!(contain,true);

}



#[test]
fn test_main(){
    let mut gw: GameWorld =GameWorld::new( 10,20);
    gw.update_mscreen();

    for i in 0..15{

        gw.enter_input('k', collision);
        gw.enter_input('a', collision);
        gw.enter_input('k', collision);
        gw.enter_input('d', collision);
        gw.move_world(collision);
        if gw.end{
            break;
        }
    }
    let objs = gw.get_obj();
    assert_eq!(0 ,0);

}



#[test]
fn teste_life(){
    let mut gw: GameWorld =GameWorld::new( 10,20);
    gw.update_mscreen();
    gw.points = 100;
    gw.gain_life();
    
    //collision( , GameObjectClass::Player(Player::new((1,1))),pnts);
}

#[test]
fn test_reset(){
    let mut gw: GameWorld =GameWorld::new( 10,20);
    gw.update_mscreen();
    gw.objects = Vec::new();
    assert_eq!(gw.objects.len(),0);
    //gw.reset();
    gw.generate_player((0,0));
    gw.move_world(collision);
    assert_ne!(gw.objects.len(),1);
    
}

fn collision(obj1:GameObjectClass,obj2:GameObjectClass,points:&mut i32)->errors::CollisionErr{
    let nobject: GameObjectClass = GameObjectClass::Base( game_object::Base::new((0i8,0i8)) );
    match obj1{
        GameObjectClass::Alien(a)=>{
            match obj2{
                GameObjectClass::Shot(s)=>{
                    if s.dir == Direction::Up{
                        *points = *points +5;
                        return errors::CollisionErr::Die;

                    }
                    else{
                        return errors::CollisionErr::Ok(GameObjectClass::Alien(a))
                    }
                },
                GameObjectClass::Base(b)=>{
                    return errors::CollisionErr::Ok(GameObjectClass::Alien( a));
                },
                _=>{
                    return errors::CollisionErr::Err;
                },

            }




        },
        GameObjectClass::Base(b)=>{
            match obj2{
                GameObjectClass::Shot(s)=>{
                    if b.get_img() ==game_object::GameImages::SpaceGuide.value(){
                        return errors::CollisionErr::Ok( GameObjectClass::Base(b));
                    }
                    else{
                        return errors::CollisionErr::Ok(GameObjectClass::Shot(s) );
                    }
                },
                GameObjectClass::Base(bb)=>{
                    return errors::CollisionErr::Ok(GameObjectClass::Base(b));
                }
                GameObjectClass::Alien(a)=>{
                    return errors::CollisionErr::Ok(GameObjectClass::Alien(a));
                },
                GameObjectClass::Player(p)=>{
                    return errors::CollisionErr::Ok(GameObjectClass::Player(p));
                }

            }
        },
        GameObjectClass::Player(p)=>{
          match obj2{
            GameObjectClass::Shot(s)=>{
                if s.dir == Direction::Down{
                        *points = *points -10;
                        return errors::CollisionErr::Die;
                    }
                    else{
                        return errors::CollisionErr::Err;
                }
            },
            GameObjectClass::Alien(a)=>{
                return errors::CollisionErr::Err;
            },
            GameObjectClass::Base(b)=>{
                return errors::CollisionErr::Ok(GameObjectClass::Player(p));
            },
            _=>{
                return errors::CollisionErr::Err;
            },
          }  
        },
        GameObjectClass::Shot(s)=>{
            match obj2{
                GameObjectClass::Alien(a)=>{
                    if s.dir == Direction::Up{
                        *points = *points +5;
                        return errors::CollisionErr::Die;
                    }
                    else{
                        return errors::CollisionErr::Ok(GameObjectClass::Alien(a));
                    }
                },
                GameObjectClass::Base(b)=>{
                    
                    return errors::CollisionErr::Ok(GameObjectClass::Shot(s));
    
                },
                GameObjectClass::Player(p)=>{
                    if s.dir == Direction::Down{
                        *points = *points -10;
                        return errors::CollisionErr::Die;
                    }
                    else{
                        return errors::CollisionErr::Err;
                    }
                }
                GameObjectClass::Shot(ss)=>{
                    if s.dir != ss.dir{
                        *points= *points+3;
                        return errors::CollisionErr::Die;
                    }
                    else{
                        return errors::CollisionErr::Ok(GameObjectClass::Shot(ss));
                    }                    
                    
                }
            }
        },

    }

}

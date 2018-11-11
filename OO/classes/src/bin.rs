extern crate classes;
extern crate termion;
extern crate eventual;
extern crate termios;

use eventual::{Future, Async};
use termion::color;
use termion::clear;
use std::time::{Instant, Duration};

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::io;
use std::io::Read;
use std::io::Write;
use std::char;

use classes::game_world::GameWorld;
use classes::game_world::GameWorldT;
use classes::game_world::errors::*;
use classes::game_world::game_object;
use classes::game_world::Direction;
use classes::game_world::game_object::GameObjectClass;
use classes::game_world::game_object::GameImagesT;
use classes::game_world::game_object::BaseT;
use classes::game_world::game_screen::GameScreenT;
use classes::game_world::game_object::GameObjectT;
use classes::game_world;

static FPS:u64 = 1;

fn main (){
    let dur = Duration::from_millis(1000/FPS);
    let mut handle = Future::spawn(move|| read_input());

    let mut gw: GameWorld =GameWorld::new( 10,20);
    gw.update_mscreen();
    
    for i in 0..15{
        print_screen(&mut gw);
        let crono: Instant = Instant::now();

        loop{
            if crono.elapsed() > dur{
                gw.move_world(collision);
                break;
            }
            if handle.is_ready() {
                let data =handle.expect().unwrap();
                gw.enter_input(data,collision);

                handle = Future::spawn(move|| read_input());
                print_screen(&mut gw);
            }
            
        }        
        //gw.move_world(collision);
        

        
    }
    


    

    //while true{
//        
        
  /*      loop{
            if crono.elapsed() > dur{
                print_screen( gw.get_screen());
                break;
            }
            
        }*/

    //}
  
}

fn print_screen(gw: &mut GameWorld){
    println!("{}", clear::All);
    if !gw.get_end(){
        gw.update_mscreen(); 
        println!("{green}{points}{reset}",green = color::Fg(color::Green),points = gw.get_points() ,reset = color::Fg(color::Reset));
    }    
    let screen = gw.get_screen();
    
    
    for line in screen{
        let str_scr:String = line.into_iter().collect();
        println!("{}",str_scr);
    }
    if gw.get_end(){
        finish_game();
    }
}

fn collision(obj1:GameObjectClass,obj2:GameObjectClass,points:&mut i32)->CollisionErr{
    let nobject: GameObjectClass = GameObjectClass::Base( game_object::Base::new((0i8,0i8)) );
    match obj1{
        GameObjectClass::Alien(a)=>{
            match obj2{
                GameObjectClass::Shot(s)=>{
                    if s.dir == Direction::Up{
                        *points = *points +5;
                        return CollisionErr::Die;

                    }
                    else{
                        return CollisionErr::Ok(GameObjectClass::Alien(a))
                    }
                },
                GameObjectClass::Base(b)=>{
                    return CollisionErr::Ok(GameObjectClass::Alien( a));
                },
                _=>{
                    return CollisionErr::Err;
                },

            }




        },
        GameObjectClass::Base(b)=>{
            match obj2{
                GameObjectClass::Shot(s)=>{
                    if b.get_img() ==game_object::GameImages::SpaceGuide.value(){
                        return CollisionErr::Ok( GameObjectClass::Base(b));
                    }
                    else{
                        return CollisionErr::Ok(GameObjectClass::Shot(s) );
                    }
                },
                GameObjectClass::Base(bb)=>{
                    return CollisionErr::Ok(GameObjectClass::Base(b));
                }
                GameObjectClass::Alien(a)=>{
                    return CollisionErr::Ok(GameObjectClass::Alien(a));
                },
                GameObjectClass::Player(p)=>{
                    return CollisionErr::Ok(GameObjectClass::Player(p));
                }

            }
        },
        GameObjectClass::Player(p)=>{
          match obj2{
            GameObjectClass::Shot(s)=>{
                if s.dir == Direction::Down{
                        *points = *points -10;
                        return CollisionErr::Die;
                    }
                    else{
                        return CollisionErr::Err;
                }
            },
            GameObjectClass::Alien(a)=>{
                return CollisionErr::Err;
            },
            GameObjectClass::Base(b)=>{
                return CollisionErr::Ok(GameObjectClass::Player(p));
            },
            _=>{
                return CollisionErr::Err;
            },
          }  
        },
        GameObjectClass::Shot(s)=>{
            match obj2{
                GameObjectClass::Alien(a)=>{
                    if s.dir == Direction::Up{
                        *points = *points +5;
                        return CollisionErr::Die;
                    }
                    else{
                        return CollisionErr::Ok(GameObjectClass::Alien(a));
                    }
                },
                GameObjectClass::Base(b)=>{
                    
                    return CollisionErr::Ok(GameObjectClass::Shot(s));
    
                },
                GameObjectClass::Player(p)=>{
                    if s.dir == Direction::Down{
                        *points = *points -10;
                        return CollisionErr::Die;
                    }
                    else{
                        return CollisionErr::Err;
                    }
                }
                GameObjectClass::Shot(ss)=>{
                    if s.dir != ss.dir{
                        *points= *points+3;
                        return CollisionErr::Die;
                    }
                    else{
                        return CollisionErr::Ok(GameObjectClass::Shot(ss));
                    }                    
                    
                }
            }
        },

    }

}

fn finish_game(){
    std::process::exit(3);
}

fn read_input() -> char{
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    let a: char= char::from(buffer[0]);
    //println!("You have hit: {}", a);
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to 
                                                    // original termios data
    return a;
}
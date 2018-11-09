use super::game_object::GameObjectT;
use super::game_object::GameImagesT;
use super::game_object::AlienT;
use super::game_object::PlayerT;
use super::game_object::BaseT;
use super::game_object::GameObjectClass;
use super::game_object::AliensClass;
use super::game_object::Alien;
use super::game_object::Player;
use super::game_object::GameImages;



pub struct GameScreen{
    screen:Vec<Vec<char>>,
    down_limit:i8,
    right_limit:i8,
}

pub trait GameScreenT{
    fn new(_down_limit:i8,_right_limit:i8)->GameScreen;
    fn update_screen(& mut self,&Vec<GameObjectClass>);
    fn get_screen(&self) -> Vec<Vec<char>>;
    fn get_limits(&self)->(i8,i8);
    fn end_screen(&mut self, win:bool, points:u32);
}


impl GameScreenT for GameScreen{
    fn get_screen(&self)->Vec<Vec<char>>{
        self.screen.clone()
    }

    fn new(_down_limit:i8,_right_limit:i8)->GameScreen{
        let v:Vec<Vec<char>> =  vec![vec![GameImages::Blank.value();_right_limit as usize];_down_limit as usize];
        GameScreen{
            down_limit: _down_limit,
            right_limit:_right_limit,
            screen:v,
        }
    } 

    fn update_screen(& mut self,objects:&Vec<GameObjectClass>){
        let mut new_screen:Vec<Vec<char>> = vec![vec![' ';(self.right_limit+1i8) as usize];(self.down_limit+1i8)  as usize];
           
        for obj in objects{
            let img:char = obj.get_img();
            let position:(i8,i8) = obj.get_position();
            new_screen[position.0 as usize][position.1 as usize] = img;
            
        }
        
        for i in 0..self.down_limit-1{
            for j in 0..self.right_limit{
                let cur_char = new_screen[i as usize][j as usize];
                match cur_char{
                    a if a == GameImages::Alien.value()=>{},
                    //p if p == GameImages::Player.value()=>{},
                    s if s == GameImages::Shot.value()=>{},
                    sa if sa == GameImages::SAlien.value()=>{},
                    p if p == GameImages::Player.value()=>{},
                    ashot if ashot == GameImages::AShot.value()=>{},
                    _=>{
                        new_screen[i as usize][j as usize] = GameImages::Blank.value();
                    }
                }
            }
            
        }
        for i in 0..self.right_limit{
            let cur_char = new_screen[(self.down_limit-1i8) as usize][i as usize];
            match cur_char{
                p if p == GameImages::Player.value()=>{},
                _=>{
                    new_screen[(self.down_limit-1i8) as usize][i as usize] = GameImages::SpaceGuide.value();
                }
            }
        }
        

        self.screen =new_screen;  
    }
    
    fn get_limits(&self)->(i8,i8){
        let dl = self.down_limit;
        let rl = self.right_limit;
        (dl,rl)
    }
    fn end_screen(&mut self,win: bool, points: u32){
        self.screen = Vec::new();
        
        self.screen.push(String::from("GAME OVER").chars().collect() );
        self.screen.push(points.to_string().chars().collect() );
        
    }
}


#[test]
fn test_update_screen() {
    let mut sc = GameScreen::new(3i8, 2i8);
    let mut go_list: Vec<GameObjectClass> = Vec::new();
    let al_pos = (0i8,0i8);
    let pl_pos = (2i8,1i8);
    go_list.push(GameObjectClass::Alien( AliensClass::Alien( Alien::new_img(GameImages::Alien.value(), al_pos ) ) )  );
    go_list.push(GameObjectClass::Player( Player::new_img(GameImages::Player.value(), pl_pos ) ) );
    sc.update_screen(&go_list);
    let screen = sc.get_screen();
    for i in 0..sc.down_limit-1{
        let mut s = &screen[i as usize];
        let ch:String =s.into_iter().collect();
        for j in 0..sc.right_limit{
            
            if (i == al_pos.0 )&& j == al_pos.1{
                
                assert_eq!(screen[i as usize][j as usize],GameImages::Alien.value());
            }
            else{
                assert_eq!(screen[i as usize][j as usize],GameImages::Blank.value());
            }
        }
        
    }
    let i = sc.down_limit-1;
    for j in 0..sc.right_limit{
        if(i,j) == pl_pos{
            assert_eq!(screen[i as usize][j as usize],GameImages::Player.value());
        }
        else{
            assert_eq!(screen[i as usize][j as usize],GameImages::SpaceGuide.value());
        }
    }
    
}
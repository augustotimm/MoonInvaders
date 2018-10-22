
extern crate termion;
extern crate termios;
extern crate eventual;


use std::{thread, time};
use time::{Instant, Duration};
use termion::color;
use termion::clear;
use eventual::{Future, Async};

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::io;
use std::io::Read;
use std::io::Write;
use std::char;




 static mut MOVEMENT_DIRECTION:bool = true;

 static ALIEN: char = 'A';

 static BLANK_SPACE:char = ' ';
 static FPS: u64 = 1;
 static PLAYER: char = 'V';

 static LEFT:char = 'a';
 static RIGHT:char = 'd';
 static SHOOT:char = 'k';

 static SHOT_OBJ:char = '!';

 

fn main() {
    
    let mut alien_screen: Vec<String> = initialize_screen();
    let mut player_line = init_player();
    //let s = transpose_string_vec(screen_arr);
    
    let mut handle = Future::spawn(move|| read_input());
    let ceiling = String::from("------------------------------");
    loop{
        
        //clear_terminal();
        let crono: Instant = Instant::now();
        println!("{}", clear::All);
        let mut pr_screen = alien_screen.clone();
        pr_screen.push(player_line.clone());
        pr_screen.insert(0, ceiling.clone());
        print_game(pr_screen);
        let dur = Duration::from_millis(1000/FPS);
        //thread::sleep(dur);
        //read_input();
         

        loop{
            if crono.elapsed() >dur{
                alien_screen = move_shots(alien_screen.clone());
                unsafe{
                    alien_screen =move_aliens(alien_screen.clone(),MOVEMENT_DIRECTION);
                }
                break;
                
            }
            if handle.is_ready(){
                let data =handle.expect().unwrap();
                handle = Future::spawn(move|| read_input());
                let a = 'a';
                match data{
                    d if d == RIGHT=>{
                        
                        let mut pl = player_line.clone();
                        let mut chars_p = pl.chars();
                        let line_temp  = move_right(chars_p.next(),chars_p.clone(),'-');
                        
                        if line_temp.is_empty(){
                        
                            continue;
                        }
                        player_line = line_temp;

                    },
                    a if a == LEFT=>{
                        let pl = player_line.clone();
                        let mut chars_p = pl.chars();
                        let line_temp = move_left(chars_p.next().clone(),chars_p.clone(),'-');
                        if line_temp.is_empty(){
                        
                            continue;
                        }
                        player_line = line_temp;
                    },
                    k if k == SHOOT =>{
                        let pl = create_shot(player_line.clone());
                        let mut chars_p = pl.chars();
                        //let line_temp = move_left(chars_p.next().clone(),chars_p.clone(),'-');
                        let mut screen_temp = alien_screen.clone();
                        screen_temp.push(pl.clone());
                        //drop(line_temp);
                        let mut tr_screen = transpose_string_vec(screen_temp.clone());
                        let _fn: fn(String, &fn(char)->char)-> String = move_one_char;
                        let _fn2: fn(char)-> char = |y:char|if y == ALIEN{BLANK_SPACE}else{SHOT_OBJ};
                        tr_screen = move_shot(tr_screen.clone(), SHOT_OBJ, &_fn,&_fn2);
                        screen_temp = transpose_string_vec(tr_screen.clone());     
                        screen_temp.pop();
                        alien_screen = screen_temp;
                        

                    },
                    _=>{
                        continue;

                    },
                }
               
                println!("{}", clear::All);
                let mut pr_screen = alien_screen.clone();
                pr_screen.push(player_line.clone());
                print_game(pr_screen);
            }
            if check_victory(alien_screen.clone()){
                end_game(true);
            }
            
         
        
        }
        
        
        //read_input();
    }

    
    
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

fn init_player()-> String{
    String::from(String::from("---------------V--------------"))
}




fn end_game(win:bool){
    if !win{
        println!("GAME OVER");
        std::process::exit(0);
    }
    else{
        println!("YOU WIN");
        std::process::exit(1);
    }
    
}


fn move_aliens(screen:Vec<String>, direction:bool)->Vec<String>{
    let mut buf:Vec<String> = Vec::new();
    if direction{
        buf = move_whole_right(screen.clone(), buf);
        if buf.is_empty(){
            let tr_screen = transpose_string_vec(screen.clone());
            buf = move_whole_right(tr_screen, buf.clone());
            if buf.is_empty(){
                end_game(false);
            }
            buf  = transpose_string_vec(buf.clone());
        }
        return buf;
    }
    else{
        buf = move_whole_left(screen.clone(), buf);
        if buf.is_empty(){
            let  tr_screen = transpose_string_vec(screen.clone());
            buf = move_whole_right(tr_screen, buf.clone());
            if buf.is_empty(){
                end_game(false);
            }            
            buf  = transpose_string_vec(buf.clone());
        }
        return buf;
    }

    fn move_whole_right(mut screen_rest:Vec<String>, mut moved_screen:Vec<String>)->Vec<String>{
        
        if screen_rest.is_empty(){
            return moved_screen;
        }
        else{
            let s = screen_rest.clone();
            let mut chars = s[0].chars();
            let temp=move_right(chars.next(), chars.clone(),BLANK_SPACE);
            if temp.is_empty(){
                unsafe{
                    MOVEMENT_DIRECTION = false;
                }                
                return Vec::new();
            }
            moved_screen.push(temp);
            screen_rest.remove(0);
            return move_whole_right(screen_rest.clone(), moved_screen);
        }
        
    }

    fn move_whole_left(mut screen_rest:Vec<String>, mut moved_screen:Vec<String>)->Vec<String>{
        
        if screen_rest.is_empty(){
            return moved_screen;
        }
        else{
            let s = screen_rest.clone();
            let mut chars = s[0].chars();
            let temp=move_left(chars.next(), chars.clone(),BLANK_SPACE);
            if temp.is_empty(){
                unsafe{
                    MOVEMENT_DIRECTION = true;
                }                
                return Vec::new();
            }
            moved_screen.push(temp);
            screen_rest.remove(0);
            return move_whole_left(screen_rest, moved_screen);
        }
        
    }

}


fn initialize_screen()-> Vec<String>{
    let mut vec = Vec::new();

    //vec.push(String::from("------------------------------"));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from("                              "));
    vec.push(String::from("                              "));

    return vec;
    
}
fn move_right(indiv: Option<char>,mut  _line:std::str::Chars<'_>, new_char:char)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
            buf.push(new_char);
            buf.push(x);
            return move_rest_right(_line.next().clone(), _line.clone(), buf.clone());
            
        } ,
        None=> buf = String::new(),
    }
    buf
}

fn move_rest_right(indiv: Option<char>, mut original_line:std::str::Chars<'_>, mut new_line:String)-> String{
    match indiv{
        Some(x)=>{
            let last = new_line.pop();
            match last{
                Some(l)=>{
                    if (l == ALIEN) &&(x ==  SHOT_OBJ){
                        new_line.push(BLANK_SPACE);
                        new_line.push(BLANK_SPACE);
                    }
                    else{
                        new_line.push(l);
                        new_line.push(x);                        
                    }
                },
                None=> return String::new(),
            }

            
            let mut rest = move_rest_right(original_line.next().clone(), original_line.clone(), new_line.clone());
            
            return rest;
            //new_line.push_str(&rest);
            //println!("{}",new_line);
            //return new_line;
            
        },
        None=>{
            let last =new_line.pop();
            
            match last{
                Some(x)=>{
                    if (x == ALIEN) || (x == PLAYER) {
                        return String::new();
                    }                    
                },
                None=>{
                    return String::new();
                }

            }
            return new_line;
        },
    }


    
}

fn move_left(indiv: Option<char>,mut  _line:std::str::Chars<'_>, new_char: char)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
            if  (x == ALIEN) || (x == PLAYER){
               return String::new(); 
            }
            return move_rest_left(_line.next().clone(), _line.clone(), buf.clone(), new_char);
            
        } ,
        None=> return String::new(),
    }
}

fn move_rest_left(indiv: Option<char>, mut original_line:std::str::Chars<'_>, mut new_line:String, new_char:char)-> String{
    match indiv{
        Some(x)=>{
            let last = new_line.pop();
            match last{
                Some(l)=>{
                    if (l == SHOT_OBJ ) &&(x ==  ALIEN){
                        new_line.push(BLANK_SPACE);
                        new_line.push(BLANK_SPACE);
                    }
                    else{
                        new_line.push(l);
                        new_line.push(x);
                    }
                },
                None=> new_line.push(x),
            }
            
            let mut rest = move_rest_left(original_line.next().clone(), original_line.clone(), new_line.clone(), new_char);
            
            return rest;
            //new_line.push_str(&rest);
            //println!("{}",new_line);
            //return new_line;
            
        },
        None=>{
            new_line.push(new_char);
            return new_line;
        },
    }


    
}

fn transpose_string_vec(s_vec:Vec<String>)-> Vec<String>{
    let mut s_char = stringvec_to_charvec(s_vec.clone());
    
    let mut result = transpose_vec(s_char.clone());
    let res = charvec_to_stringvec(result);
    return res;

}


fn stringvec_to_charvec(mut s:Vec<String>)-> Vec<Vec<char>>{
    if s.len()<1{
        return Vec::new();
    }
    else{
        let mut buf:Vec<Vec<char>> = Vec::new();
        buf.push(s[0].chars().collect());
        s.remove(0);
        let mut rest =stringvec_to_charvec(s.clone());
        buf.append(&mut rest);
            
        return buf;
    }
}

fn transpose_vec(v:Vec<Vec<char>>)-> Vec<Vec<char>>{
    let mut buf:Vec<Vec<char>> = Vec::new();
    
    buf = init_transpose(v.clone(),buf, 0);

    fn transpose(v:Vec<Vec<char>>,mut buffer:Vec<Vec<char>>, c:usize, l:usize)-> Vec<Vec<char>>{
        
        if l>= v.len() {
            return buffer;
        }
        else{
            if c>=v[l].len(){
                return transpose(v, buffer, 0, l+1);
            }
            else{
                buffer[c][l] = v[l][c];
                
                
                return transpose(v.clone(), buffer.clone(), c+1, l);
            }
        }

    }
    fn init_transpose(v:Vec<Vec<char>>,mut buf:Vec<Vec<char>>, iter:usize)->Vec<Vec<char>>{
        if iter>= v[0].len(){
            return buf;
        }
        else{
            
            buf.push(vec![BLANK_SPACE;v.len()]);
            return init_transpose(v.clone(), buf.clone(), iter+1);
        }
    }

    buf = transpose(v.clone(), buf.clone(), 0, 0);
    return buf;
}

fn charvec_to_stringvec(mut vec_char:Vec<Vec<char>>)->Vec<String>{
    if vec_char.len()<1{
        return Vec::new();
    }
    else{
        let mut buf:Vec<String> = Vec::new();
        let v:Vec<char> =vec_char.remove(0);
        let s:String = v.into_iter().collect();
        buf.push(s);
        
        let mut rest =charvec_to_stringvec(vec_char.clone());
        buf.append(&mut rest);
            
        return buf;
    }

}

fn print_game(mut s:Vec<String>){
    if s.is_empty(){
        return;
    }
    else{
        if s.len()>=2{
            println!("{red}{game}{reset}",red = color::Fg(color::Red),game=s[0],reset = color::Fg(color::Reset));
        }
        else{
            println!("{green}{game}{reset}",green = color::Fg(color::Green),game=s[0],reset = color::Fg(color::Reset));
        }
        

        s.remove(0);
        print_game(s.clone());
    }
    
}

fn create_shot(mut player_line: String) -> String{
    if player_line.is_empty(){
        return String::new();
    }
    else{
        let first: char = player_line.remove(0);
        if first == PLAYER{
            player_line.insert(0, SHOT_OBJ);
            return player_line;
        }
        else{
            let mut temp_pl = create_shot(player_line);
            temp_pl.insert(0, first);
            return temp_pl;
        }
    }


}
fn move_shot(mut screen:Vec<String>,shot:char, move_somewhere: &fn(String, &fn(char)->char)-> String, resolve:&fn(char)->char)-> Vec<String>{   
     if screen.is_empty(){
        return Vec::new(); 
    }
    else{
        let length = screen[0].len() -1;
        let last = screen[0].remove(length);
        if last == shot{
            screen[0].insert(length,last);
            let new_line= move_somewhere(screen[0].clone(),resolve);
            
            screen.remove(0);
            screen.insert(0,new_line);
            return screen;

        }
        else{
            screen[0].insert(length,last);
            let head = screen[0].clone();
            screen.remove(0);
            let mut rest = move_shot(screen.clone(), shot, move_somewhere, resolve);
            rest.insert(0, head);
            return rest;
        }

    }

}

fn move_one_char(mut s:String, resolve:&fn(char)->char)-> String{
    let length = s.len() -1;
    let first =  s.remove(length);
    let second = s.remove(length-1);
    let result = resolve(second);
    if result == SHOT_OBJ{
        s.push(first);
        s.push(second);
    }
    else{
        s.push(result);
        s.push(BLANK_SPACE);
    }
    return s;
}

fn move_shots(screen: Vec<String>)->Vec<String>{

    let mut tr_screen = transpose_string_vec(screen.clone());
    tr_screen = move_rest_shots(tr_screen.clone());
    let buf = transpose_string_vec(tr_screen.clone());
    return buf;

    fn move_rest_shots(mut screen:Vec<String>)->Vec<String>{
        let mut rest = String::new();
        if screen.is_empty(){
            return Vec::new();
        }
        let first_string = screen.remove(0);
        if first_string.contains(SHOT_OBJ){
            rest = move_equals_left(first_string.clone(), SHOT_OBJ);
            

        }
        else{
            rest =first_string.clone();
        }
        let mut result = move_rest_shots(screen.clone());
        result.insert(0, rest);
        return result;
    }
    


}

fn move_equals_left(mut s:String, compared:char) ->String{
    if s.is_empty() {
        return s;
    }
    let mut first = s.remove(0);
    if s.is_empty() {
        s.insert(0, first);
        return s;
    }
    let mut second = s.remove(0);
    let mut move_more = true;
    if second  == compared{
        if first  != BLANK_SPACE{
            first = BLANK_SPACE;
            second = BLANK_SPACE;
            s.insert(0, second);
            move_more = s.contains(compared);
        }
        else{
            s.insert(0, first);
            first = second;
            move_more = s.contains(compared);
        }
        
    }
    else{
        s.insert(0, second);
        
    }
    let mut result = String::new();
    if move_more{
        result = move_equals_left(s, compared);
    }
    else{
        result = s;
    }
    result.insert(0, first);
    
    return result;
}

fn check_victory(alien_space:Vec<String>)->bool{    
    
    let ans:Vec<bool> =alien_space.into_iter().map(|x| return victory_condition(x)(ALIEN)).collect();
    return ! ans.contains(&true);
    fn victory_condition(vec:String)-> impl Fn(char)->bool{
       move |x| vec.contains(x)
    }
}

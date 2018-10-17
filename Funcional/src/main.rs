use std::{thread, time};


 static mut MOVEMENT_DIRECTION:bool = false;

 static ALIEN: char = 'A';

 static BLANK_SPACE:char = ' ';
 static FPS: u64 = 1;

fn main() {
    
    let mut screen_arr: Vec<String> = initialize_screen();
    
    //let s = transpose_string_vec(screen_arr);


    loop{
        unsafe{
            screen_arr =move_whole_screen(screen_arr.clone(),MOVEMENT_DIRECTION);
        }
        clear_terminal();
        print_game(screen_arr.clone());
        let dur = time::Duration::from_millis(1000/FPS);
        thread::sleep(dur);

    }

    
    /*let mut v:Vec<char> = screen_arr[0].chars().collect();
    let mut s: String = v.into_iter().collect();
    println!("S:{:?}",s);


    //let s: String = charr.into_iter().collect();
    //println!("{}",s);
    //println!("{:?}",screen);

    //let mut test = move_whole_screen(screen_arr,true);

    //println!("{:?}",test);
*/
    
}


fn clear_terminal(){
    print!("{}[2J", 27 as char); //Clear Screen

}

fn end_game(win:bool){
    if !win{
        println!("GAME OVER");
        std::process::exit(0);
    }
    else{
        std::process::exit(1);
    }
    
}


fn move_whole_screen(screen:Vec<String>, direction:bool)->Vec<String>{
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
            let temp=move_right(chars.next(), chars.clone());
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
            let temp=move_left(chars.next(), chars.clone());
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

    vec.push(String::from("------------------------------"));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from(" A A A A A A A A A A A A A    "));
    vec.push(String::from("                              "));
    vec.push(String::from("                              "));

    return vec;
    
}
fn move_right(indiv: Option<char>,mut  _line:std::str::Chars<'_>)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
            buf.push(BLANK_SPACE);
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
            new_line.push(x);
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
                    if x == ALIEN{
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

fn move_left(indiv: Option<char>,mut  _line:std::str::Chars<'_>)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
            if x == ALIEN{
               return String::new(); 
            }
            return move_rest_left(_line.next().clone(), _line.clone(), buf.clone());
            
        } ,
        None=> return String::new(),
    }
}

fn move_rest_left(indiv: Option<char>, mut original_line:std::str::Chars<'_>, mut new_line:String)-> String{
    match indiv{
        Some(x)=>{
            new_line.push(x);
            let mut rest = move_rest_left(original_line.next().clone(), original_line.clone(), new_line.clone());
            
            return rest;
            //new_line.push_str(&rest);
            //println!("{}",new_line);
            //return new_line;
            
        },
        None=>{
            new_line.push(BLANK_SPACE);
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
        println!("{}",s[0]);
        s.remove(0);
        print_game(s.clone());
    }
    
}
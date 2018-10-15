


fn main() {
    
    let mut screen_arr: Vec<String> = Vec::new();
    screen_arr.push(String::from(initialize_screen()));
    screen_arr.push(String::from(initialize_screen()));
    transpose_string_vec(screen_arr.clone());
    println!("Screen ARR{}",screen_arr[0]);
    let screen = initialize_screen();
    println!("{}",String::from(screen));
    

    let count = screen.chars().count();
    
    let mut chars = screen.chars();
    


    let mut charr:Vec<char> = screen.chars().collect();
    //let s: String = charr.into_iter().collect();
    //println!("{}",s);
    

/*
    let mut indiv_char = chars.next();

    let mut result = move_right(indiv_char.clone(), chars.clone());

    println!("{}",result);

    let mut chars_r = result.chars();

    let mut indiv_char_r = chars_r.next();
    let mut result2 = move_left(indiv_char_r.clone(), chars_r.clone());

    println!("{}",result2);
  */  


}



fn initialize_screen()-> &'static str{

    "--a--"
}
fn move_right(indiv: Option<char>,mut  _line:std::str::Chars<'_>)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
            buf.push('-');
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
            new_line.pop();
            return new_line;
        },
    }


    
}

fn move_left(indiv: Option<char>,mut  _line:std::str::Chars<'_>)-> String{
    let mut buf = String::with_capacity(5);
    match indiv{
        Some(x)=>{
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
            new_line.push('-');
            return new_line;
        },
    }


    
}

fn transpose_string_vec(s_vec:Vec<String>){
    let mut s_char = stringvec_to_charvec(s_vec.clone());
    println!("{:?}",s_char);
    let mut result = transpose_vec(s_char.clone());
    println!("{:?}",result);


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
            
            buf.push(vec!['0';v.len()]);
            return init_transpose(v.clone(), buf.clone(), iter+1);
        }
    }

    buf = transpose(v.clone(), buf.clone(), 0, 0);
    return buf;
}



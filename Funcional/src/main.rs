


fn main() {
    let mut screen_arr: Vec<String> = vec![String::new(); 126];
    screen_arr[0] = String::from(initialize_screen());
    println!("Screen ARR{}",screen_arr[0]);
    let screen = initialize_screen();
    println!("{:?}",String::from(screen));

    let count = screen.chars().count();
    
    let mut chars = screen.chars();

    let mut indiv_char = chars.next();

    let mut result = move_right(indiv_char.clone(), chars.clone());

    println!("{}",result);

    let mut chars_r = result.chars();

    let mut indiv_char_r = chars_r.next();
    let mut result2 = move_left(indiv_char_r.clone(), chars_r.clone());

    println!("{}",result2);

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
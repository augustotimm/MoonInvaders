


fn main() {

    let screen = initialize_screen();
    println!("{:?}",String::from(screen));

    let count = screen.chars().count();
    
    let mut chars = screen.chars();

    let mut indiv_char = chars.next();

    let mut result = move_right(indiv_char, chars);

    println!("{}",result);

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
            return move_rest(_line.next(), _line, buf.clone());
            
        } ,
        None=> buf = String::new(),
    }
    buf
}

fn move_rest(indiv: Option<char>, mut original_line:std::str::Chars<'_>, mut new_line:String)-> String{
    match indiv{
        Some(x)=>{
            new_line.push(x);
            let mut rest = move_rest(original_line.next(), original_line, new_line.clone());
            println!("{}", rest);
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

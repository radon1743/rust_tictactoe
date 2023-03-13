
//use std::io;

use text_io::read;
fn check_win(st:&mut [char;9])-> char{
    println!("");
    for i in 0..2{
        if st[i]==st[i+3]&&st[i+3]==st[i+6] && st[i]!='E'{
            return st[i];
        }
        else if st[(i*3)]==st[(i*3)+1] && st[(i*3)+1]==st[(i*3)+2] && st[i]!='E'{
            return st[i*3];
        }
    }
    'E'
}

fn print_state(st:[char;9]){
    for i in 1..10{
        if i%3==0{
            println!("{}  ",st[i-1]);
        }
        else {
            print!("{}  ",st[i-1]);
        }
    }
}

fn change_state(st:&mut [char;9], pos:usize,chance:i32)->i32{
    
    if chance==1 && st[pos]=='E'{
        st[pos]='X';
        return 0;
    }
    else if chance==0 && st[pos]=='E'{
        st[pos]='O';
        return 1;
    }
    else{
        println!("error");
        return 0;
    };
}

fn reset_state(st:&mut [char;9]){
    for i in 0..9{
        st[i]='E';
    }
}
fn main() {
    let mut state:[char;9]=['E';9];
    let mut chance:i32=1;
    let mut _x=0;
    let mut _o=0;
    loop {
        if chance==1{
            println!("Move! for X");
        }
        else {
            println!("Move! for O");
        }
        print_state(state);
    
        /*let mut input_text: String=String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("Enter a text");*/


        let pos: usize = read!();
        chance=change_state(&mut state, pos,chance);
        let win:char=check_win(&mut state);
        if win!='E'{
            print_state(state);
            println!("{} player won!!",win);
            reset_state(&mut state);
            if win=='X'{
                _x+=1;
            }
            else{
                _o+=1;
            }
            println!("            _________________");
            println!("Score board:|X : {} | O : {}|",_x,_o);
            println!("            ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
            println!("--Board reset--");
            
        }   
    }
}

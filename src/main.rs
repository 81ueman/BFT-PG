use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Tagged{
    pos: usize,
    types: Vec<(usize,String)>
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong when reading the file");
    
    let contents:Vec<&str> = contents.split(';').collect();
    let mut pnum = contents.iter().fold(0, |sum, s| sum + s.trim().split_ascii_whitespace().count() -1);
    eprintln!("sum of types:{}", pnum);
    assert_eq!(1, pnum%2);
    

    let mut a: Vec<String>  = Vec::new();
    let mut ty_arr: Vec<Tagged> = Vec::new(); //usize: original position in the sentence

    //constraints in step 0
    for (i,s) in contents.iter().enumerate(){
        let n = s.split_ascii_whitespace().skip(1).count();
        ty_arr.push(Tagged {pos:i,types:Vec::new()});
        for (j,typ) in s.split_ascii_whitespace().skip(1).enumerate(){
            println!("(int x_{}_{} 1 {})", i,j, n);
            println!("(int u_{}_{} -3 3)", i,j);
            a.push(s.to_string());
            ty_arr[i].types.push((j,typ.to_string()));
        }
    }
    for v in ty_arr.iter(){
        for (j,s) in v.types.iter().enumerate() {
            if let Some(pos) = v.types.iter().skip(j+1).position(|t| s==t) {
                println!("(< x_{}_{} x_{}_{})", v.pos,j,v.pos,pos+j+1); //なんかもっとキレイな書き方あるでしょ
            }
        }
    }
    println!("(int cost 0 1000)");
    print!("(= cost (+ ");
    for Tagged{pos:i,types:v} in ty_arr.iter(){
        for (j,_) in v.iter() {
            print!("(abs u_{}_{}) ", i,j);
        }
    }
    println!("))");
    println!("(objective minimize cost)");
    // external
    while pnum!=1 {
        let mut rn = 0;
        let mut ri0 = 0;
        let mut rm = 0;
        let mut rj0 = 0;
        let mut found = false;
        for Tagged {pos:_, types:v} in ty_arr.iter() {
            eprintln!("{:?}", v);
        }
        'outer: for Tagged{pos:n,types:v} in ty_arr.iter().take(ty_arr.len()-1) {
            let Tagged{pos:m, types:w} = &ty_arr[n+1];
            for (ipos,(i0,t1))in v.iter().enumerate().rev() {
                if let Some(jpos) = w.iter().position(|(_,t2)| t1 == t2) {
                    let (j0,_) = &w[jpos];
                    eprintln!("found pair! {} {} {} {}", n,i0, m, j0);
                    rn = *n;
                    ri0 = ipos;
                    rm = *m;
                    rj0 = jpos;
                    for (i,_) in v {if i!=i0 {println!("(< x_{}_{} x_{}_{})", n,i, n,i0);}}
                    for (j,_) in w {if j!=j0 {println!("(< x_{}_{} x_{}_{})", m,j0, m,j)}}
                    println!("(= (+ u_{}_{} 1) u_{}_{})", n,i0, m,j0);
                    pnum -= 2;
                    found = true;
                    break 'outer;
                }
            }
        }
        if found {
            ty_arr[rn].types.remove(ri0);
            ty_arr[rm].types.remove(rj0);
        } else {
            eprintln!("cannot finish external contraction");
            std::process::exit(1);
        }
    }

}

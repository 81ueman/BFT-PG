use std::env;
use std::fs::File;
use std::io::prelude::*;

struct TaggedWord{
    word: String,
    pos: usize,
    types: Vec<(usize,String)>
}

type Sentence = Vec<TaggedWord>;

fn name(st: &str) -> &str{
    st.split_ascii_whitespace().next().unwrap().trim()
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
    

    let mut ty_arr: Sentence = Vec::new(); //usize: original position in the sentence

    //constraints in step 0
    for (i,s) in contents.iter().enumerate(){
        let n = s.split_ascii_whitespace().skip(1).count();
        ty_arr.push(TaggedWord {word:name(s).to_string(), pos:i,types:Vec::new()});
        let word = name(s);
        for (j,typ) in s.split_ascii_whitespace().skip(1).enumerate(){
            println!("(int x_{}_{}_{} 1 {})", word,i,j, n);
            println!("(int u_{}_{}_{} -3 3)", word,i,j);
            ty_arr[i].types.push((j,typ.to_string()));
        }
    }
    for v in ty_arr.iter(){
        for (j,s) in v.types.iter().enumerate() {
            if let Some(pos) = v.types.iter().skip(j+1).position(|t| s.1==t.1) {
                println!("(< x_{}_{}_{} x_{}_{}_{})", v.word ,v.pos,j,v.word, v.pos,pos+j+1); //なんかもっとキレイな書き方あるでしょ
            }
        }
    }
    println!("(int cost 0 1000)");
    print!("(= cost (+ ");
    for TaggedWord{word,pos,types:v} in ty_arr.iter(){
        for (j,_) in v.iter() {
            print!("(abs u_{}_{}_{}) ", word,pos,j);
        }
    }
    println!("))");
    println!("(objective minimize cost)");
    // external
    while pnum!=1 {
        let mut rmv_ind = None;
        for TaggedWord { types:v,..} in ty_arr.iter() {
            eprintln!("{:?}", v);
        }
        'outer: for TaggedWord{word:word1,pos:n,types:v} in ty_arr.iter().take(ty_arr.len()-1) {
            let TaggedWord{word:word2,pos:m, types:w} = &ty_arr[n+1];
            for (ipos,(i0,t1))in v.iter().enumerate().rev() {
                if let Some(jpos) = w.iter().position(|(_,t2)| t1 == t2) {
                    let (j0,_) = &w[jpos];
                    eprintln!("found pair! {} {} {} {}", n,i0, m, j0);
                    rmv_ind = Some((*n,ipos,*m,jpos));
                    for (i,_) in v {if i!=i0 {println!("(< x_{}_{}_{} x_{}_{}_{})", word1, n,i,word1, n,i0);}}
                    for (j,_) in w {if j!=j0 {println!("(< x_{}_{}_{} x_{}_{}_{})", word2, m,j0, word2,m,j)}}
                    println!("(= (+ u_{}_{}_{} 1) u_{}_{}_{})", word1, n,i0,word2, m,j0);
                    pnum -= 2;
                    break 'outer;
                }
            }
        }
        match rmv_ind {
            Some((rn,ri0,rm,rj0)) => {
                ty_arr[rn].types.remove(ri0);
                ty_arr[rm].types.remove(rj0);
            },
            None => {
                eprintln!("cannot finish external contraction");
                std::process::exit(1);
            }
        }
    }

}

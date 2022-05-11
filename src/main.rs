use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;
use std::process::Command;

fn write_file(str: String, f: &mut BufWriter<File>){
    f.write(str.as_bytes()).unwrap();
}

fn read_file(filename:&str)  -> Result<(HashSet<String>, Vec<Vec<String>>), io::Error>{
    let mut typenum = HashSet::new();
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut sentences = Vec::new();
    for line in reader.lines(){
        let line = line?;
        let mut sent = Vec::new();
        for word in line.trim().split_ascii_whitespace(){
            typenum.insert(word.to_string());
            sent.push(word.to_string());
        }
        sentences.push(sent);
    }
    Ok((typenum,sentences))
}

fn write_vars(typenum: &HashMap<&String,usize>, f: &mut BufWriter<File>){
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            write_file(format!("(int x_{}_{} 0 10)\n", s,i),f);
            write_file(format!("(int u_{}_{} -3 3)\n", s,i),f);
        }
    }
    write_file(format!("(int xcost 0 10)\n"),f);
    write_file(format!("(nvalue xcost ("),f);
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            write_file(format!("x_{}_{} ", s,i),f);
        }
    }
    write_file(format!("))\n"),f);

    write_file(format!("(int ucost 0 30)\n"),f);
    write_file(format!("(= ucost (+ "),f);
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            write_file(format!("(abs u_{}_{}) ", s,i),f);
        }
    }
    write_file(format!("))\n"),f);

    write_file(format!("(int cost 0 20)\n"),f);
    write_file(format!("(= cost (+ xcost ucost))\n"),f);
    write_file(format!("(objective minimize cost)\n"),f);


}


fn calc_psum(typenum: &HashMap<&String,usize>, sentences: &Vec<String>) -> usize{
    sentences.iter().map(|s|  typenum.get(s).unwrap() as &usize).sum() 
}
fn infer(typenum: &HashMap<&String,usize>, sentences: &Vec<Vec<String>>, f:&mut BufWriter<File>){
    write_file(";variable definitions\n".to_string(), f);
    write_vars(&typenum,f);


    for (ith,contents) in sentences.iter().enumerate(){
        let pnum = calc_psum(typenum, contents);
        write_file(format!("\n; {}th sentences\n", ith),f);
        write_file(format!(";sum of types:{}\n", pnum),f);
        assert_eq!(1, pnum%2, "{}th sentence:the sum of primitive types should be odd", ith);
    
        let mut vars: Vec<String> = Vec::new();
        let mut inner = HashSet::new();
        for s in contents{
            let num  = *typenum.get(s).unwrap();
            let n = vars.len();
            for i in 0..num {
                vars.push(format!("{}_{}", s, i));
            }
            for i in 0..num{
                for j in (i+1..num).step_by(2){
                    inner.insert((n+i, n+j));
                }
            }
        }



        write_file(format!("(count 0 ("),f);
        for s in &vars{
            write_file(format!("x_{} ", s),f);
        }
        write_file(format!(") eq 1)\n"),f);

        //write_file(format!("(objective minimize ucost)");
        //write_file(format!("(objective minimize xcost)");

        write_file(format!(";{:?}\n", vars),f);
        for i  in 0..pnum {
            for j in (i+1..pnum).step_by(2) {
                write_file(format!("(predicate (cont_{}_{}_{}) (and (= x_{} x_{}) (= (+ u_{} 1) u_{})))\n",ith,i,j, vars[i], vars[j], vars[i], vars[j]),f);
            }
        }

        for (i,j) in inner{
            write_file(format!("(! (cont_{}_{}_{}))\n", ith,i,j),f);
        }
        for diff in (1..pnum+2).step_by(2) {
            for i in 0..pnum-diff {
                write_file(format!("(predicate (comp_{}_{}_{}) (or ", ith,i,i+diff),f);
                for j in (i+1..i+diff+1).step_by(2){
                    write_file(format!("(and (cont_{}_{}_{}) ",ith, i,j),f);
                    if i +1 != j{
                        write_file(format!("(comp_{}_{}_{}) ", ith,i+1, j-1),f);
                    }
                    if j!=i+diff {
                        write_file(format!("(comp_{}_{}_{}) ", ith,j+1,i+diff),f);
                    }
                    write_file(format!(")"),f);
                }
                write_file(format!("))\n"),f);
            }
        }
        write_file(format!("(or "),f);
        for spos in (0..pnum).step_by(2){
            write_file(format!("(and "),f);
            write_file(format!("(= x_{} 0) ", vars[spos]),f);
            if spos != 0{write_file(format!("(comp_{}_0_{}) ", ith,spos-1),f)}
            if spos != pnum-1{write_file(format!("(comp_{}_{}_{})", ith,spos+1, pnum-1),f)}
            write_file(format!(")"),f);
        }
        write_file(format!(")\n"),f);
    }

}

fn main() -> io::Result<()>{
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let (words_dic,sentences) = read_file(filename).unwrap();
    println!(";{:?}", words_dic);
    println!(";{:?}", sentences);
    let words_num = words_dic.len();
    println!(";{} kinds of words", words_num);
    println!();

    let mut pat = 0;
    for sum in 0..2{
        'outer: for perm in (0..words_num).combinations_with_replacement(sum){
            //ewrite_file(format!("{:?}", perm);
            let mut typenum = HashMap::new();
            for (i,word) in words_dic.iter().enumerate(){
                typenum.insert(word, perm.iter().filter(|&n| *n==i).count()+1);
            }
            for sent in sentences.iter() {
                let sum:usize = sent.iter().map(|w| typenum.get(w).unwrap()).sum();
                if sum%2 != 1 {continue 'outer}
                //if sum%2 != 1 {break}
            }
            let mut f = BufWriter::new(File::create("tmp.cps").unwrap());
            write_file(format!(";{:?}\n", typenum), &mut f);
            write_file(format!(";{}th pattern\n\n" ,pat),&mut f);
            eprintln!("{}th pattern", pat);
            pat+=1;
            infer(&typenum, &sentences, &mut f);
            f.flush()?;
            f.into_inner().unwrap().sync_all()?;
            let child = Command::new("sugar")
            .args(&["tmp.cps"])
            .output()
            .expect("failed to start `sugar`");
            let stdout = child.stdout;
            let result = String::from_utf8(stdout).unwrap();
            let satisfiable = result.split_terminator('\n').next().unwrap().split_ascii_whitespace().nth(1).unwrap() == "SATISFIABLE";
            println!("{}\n", satisfiable);
            if satisfiable{
                let iter = result.split_terminator('\n').rev().take_while(|s| s.chars().nth(0).unwrap() != 'o').skip(5);
                for s in iter {
                    println!("{}", s);
                }
                break;
            }
        }

        
        /*let reader = BufReader::new(stdout);
        for line in reader.lines(){
            println!("{}", line?);
        }*/
    }


    Ok(())
}

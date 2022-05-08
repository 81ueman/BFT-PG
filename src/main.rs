use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

fn init_vars(filename:&str)  -> Result<HashMap<String,usize>, io::Error>{
    let mut typenum = HashMap::new();
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        for iter in line.split(';'){
            let word = iter.split_ascii_whitespace().next().unwrap();
            let num:usize = iter.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
            typenum.insert(word.to_string(),num);
        }
    }
    Ok(typenum)
}

fn write_vars(typenum: &HashMap<String,usize>){
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            println!("(int x_{}_{} 0 10)", s,i);
            println!("(int u_{}_{} -3 3)", s,i);
        }
    }
    println!("(int xcost 0 10)");
    print!("(nvalue xcost (");
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            print!("x_{}_{} ", s,i);
        }
    }
    println!("))");

    println!("(int ucost 0 30)");
    print!("(= ucost (+ ");
    for (s,n) in typenum.iter() {
        for i in 0..*n{
            print!("(abs u_{}_{}) ", s,i);
        }
    }
    println!("))");

    println!("(int cost 0 20)");
    println!("(= cost (+ xcost ucost))");
    println!("(objective minimize cost)");


}

fn main() -> io::Result<()>{
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let typenum = init_vars(filename).unwrap();


    write_vars(&typenum);

    eprintln!("{:?}", typenum);

    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    for (ith,line) in reader.lines().enumerate(){
        let line = line.unwrap();
        let contents:Vec<&str> = line.split(';').collect();
        let pnum:usize = contents.iter().fold(0, |sum, s| sum + (s.trim().split_ascii_whitespace().nth(1).unwrap().parse() as Result<usize,_>).unwrap());//どう書けばいいの？
        println!();
        println!("; {}th sentences", ith);
        eprintln!("sum of types:{}", pnum);
        assert_eq!(1, pnum%2, "{}th sentence:the sum of primitive types should be odd", ith);
    
        let mut vars: Vec<String> = Vec::new();
        let mut inner = HashSet::new();
        for s in contents{
            let mut iter = s.split_ascii_whitespace();
            let word = iter.next().unwrap();
            let num  = iter.next().unwrap().parse().unwrap();
            let n = vars.len();
            for i in 0..num {
                vars.push(format!("{}_{}", word, i));
            }
            for i in 0..num{
                for j in (i+1..num).step_by(2){
                    inner.insert((n+i, n+j));
                }
            }
        }



        print!("(count 0 (");
        for s in &vars{
            print!("x_{} ", s);
        }
        println!(") eq 1)");

        //println!("(objective minimize ucost)");
        //println!("(objective minimize xcost)");

        eprintln!("{:?}", vars);
        for i  in 0..pnum {
            for j in (i+1..pnum).step_by(2) {
                println!("(predicate (cont_{}_{}_{}) (and (= x_{} x_{}) (= (+ u_{} 1) u_{})))",ith,i,j, vars[i], vars[j], vars[i], vars[j]);
            }
        }

        for (i,j) in inner{
            println!("(! (cont_{}_{}_{})) ", ith,i,j);
        }
        for diff in (1..pnum+2).step_by(2) {
            for i in 0..pnum-diff {
                print!("(predicate (comp_{}_{}_{}) (or ", ith,i,i+diff);
                for j in (i+1..i+diff+1).step_by(2){
                    print!("(and (cont_{}_{}_{}) ",ith, i,j);
                    if i +1 != j{
                        print!("(comp_{}_{}_{}) ", ith,i+1, j-1);
                    }
                    if j!=i+diff {
                        print!("(comp_{}_{}_{}) ", ith,j+1,i+diff);
                    }
                    print!(")");
                }
                println!("))");
            }
        }
        print!("(or ");
        for spos in (0..pnum).step_by(2){
            print!("(and ");
            print!("(= x_{} 0) ", vars[spos]);
            if spos != 0{print!("(comp_{}_0_{}) ", ith,spos-1)}
            if spos != pnum-1{print!("(comp_{}_{}_{})", ith,spos+1, pnum-1)}
            print!(")");
        }
        println!(")");
    }


    Ok(())
}

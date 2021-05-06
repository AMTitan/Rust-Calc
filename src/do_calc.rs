extern crate regex;
use regex::Regex;
pub fn calc(input: &String) {
    let mut re = Regex::new(r"[*]").unwrap();
    let multi = re.replace_all(&input, "|*|");
    re = Regex::new(r"[/]").unwrap();
    let divide = re.replace_all(&multi, "|/|");
    re = Regex::new(r"[+]").unwrap();
    let add = re.replace_all(&divide, "|+|");
    re = Regex::new(r"[-]").unwrap();
    let add = re.replace_all(&add, "|-|");
    let mut split: Vec<&str> = add.split("|").collect();
    for x in 0..split.len() {
        if (split[x] == "*") {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap() * split[x+1].parse::<i32>().unwrap()).to_string().into_boxed_str());
            
        }
    }
    println!("num : {:?}", split);
    print!("calc (do ctrl+c or cmd+c to stop): {:?}", &split);
}
extern crate regex;
use regex::Regex;
pub fn calc(input: &String) {
    let mut re = Regex::new(r"[*]").unwrap();
    let multi = re.replace_all(&input, "|*|");
    re = Regex::new(r"[/]").unwrap();
    let divide = re.replace_all(&multi, "|/|");
    re = Regex::new(r"[+]").unwrap();
    let add = re.replace_all(&divide, "|+|");
    re = Regex::new(r"[&^]").unwrap();
    let power = re.replace_all(&add, "|^|");
    re = Regex::new(r"[-]").unwrap();
    let add = re.replace_all(&power, "|-|");
    let mut split: Vec<&str> = power.split("|").collect();
    let mut length = split.len();
    let mut x = 0;
    while x < length {
        if split[x] == "^" {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap().pow(split[x + 1].parse::<i32>().unwrap() as u32)).to_string().into_boxed_str());
            split[x-1] = "";
            split[x+1] = "";
            let mut new: Vec<&str> = vec![];
            for y in 0..split.len() {
                if split[y] != "" {
                    new.push(split[y]);
                }
            }
            split = new;
            x = 0;
            length = split.len();
        }else {
            x = x+1;
        }
    }
    length = split.len();
    x = 0;
    while x < length {
        if split[x] == "*" {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap() * split[x+1].parse::<i32>().unwrap()).to_string().into_boxed_str());
            split[x-1] = "";
            split[x+1] = "";
            let mut new: Vec<&str> = vec![];
            for y in 0..split.len() {
                if split[y] != "" {
                    new.push(split[y]);
                }
            }
            split = new;
            x = 0;
            length = split.len();
        }else if split[x] == "/" {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap() / split[x+1].parse::<i32>().unwrap()).to_string().into_boxed_str());
            split[x-1] = "";
            split[x+1] = "";
            let mut new: Vec<&str> = vec![];
            for y in 0..split.len() {
                if split[y] != "" {
                    new.push(split[y]);
                }
            }
            split = new;
            x = 0;
            length = split.len();
        }else {
            x = x+1;
        }
    }
    length = split.len();
    x = 0;
    while x < length {
        if split[x] == "+" {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap() + split[x+1].parse::<i32>().unwrap()).to_string().into_boxed_str());
            split[x-1] = "";
            split[x+1] = "";
            let mut new: Vec<&str> = vec![];
            for y in 0..split.len() {
                if split[y] != "" {
                    new.push(split[y]);
                }
            }
            split = new;
            x = 0;
            length = split.len();
        }else if split[x] == "-" {
            split[x] = Box::leak((split[x-1].parse::<i32>().unwrap() - split[x+1].parse::<i32>().unwrap()).to_string().into_boxed_str());
            split[x-1] = "";
            split[x+1] = "";
            let mut new: Vec<&str> = vec![];
            for y in 0..split.len() {
                if split[y] != "" {
                    new.push(split[y]);
                }
            }
            split = new;
            x = 0;
            length = split.len();
        }else {
            x = x+1;
        }
    }
    print!("calc (do ctrl+c or cmd+c to stop): {:?}", &split[0]);
}
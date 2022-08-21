use std::env::Args;
use std::fs::OpenOptions;
use std::{process, env};
use std::{fs::File, collections::HashMap};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use regex::{Regex, Captures};

type IndexMap<'a> = HashMap<&'a str, i32>;

fn read_file(path: &Path) -> Result<String, String> {
    let mut file: File;
    match OpenOptions::new().read(true).open(path) {
        Ok(f) => file = f,
        Err(e) => {
            return Err(format!("Cannot open {:?}! \n{}", &path, e))
        }
    }

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("Error reading file! \n{}", e))
    }

    Ok(contents)
}

fn read_jumps<'a>(input: &'a String, regex: &'a Regex) -> IndexMap<'a> {
    let mut index = 0;
    let mut map: IndexMap = HashMap::new();

    for line in input.lines() {
        let captures:Captures;
        match regex.captures(line) {
            Some(r) => captures = r,
            None => continue,
        }

        let jump = captures.get(1).unwrap().as_str();
        if !map.contains_key(&jump) {
            map.insert(jump.clone(), index);
        }
        index += 1;
    }

    map
}

fn compile<'a>(input: &String, indices: &'a IndexMap, regex: &Regex) -> String {
    let mut compiled = String::new();
    let mut line_num = 0;

    for line in input.lines() {
        if let Some(label) = indices.get((line_num).to_string().as_str()) {
            compiled.push_str(format!("jump_{}:", label).as_str());
            compiled.push_str("\n");
        }

        match regex.captures(line) {
            Some(r) => {
                compiled.push_str("jump ");
                let index = indices.get(&r.get(1).unwrap().as_str()).unwrap();
                let args = r.get(2).unwrap().as_str();
                compiled.push_str(format!("jump_{} ", index).as_str());
                compiled.push_str(args);
            },
            None => compiled.push_str(line)
        }
        compiled.push_str("\n");

        line_num += 1;
    }

    compiled
}

fn write_file(path: &Path, contents: String) -> Result<String, String> {
    let mut file: File = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Cannot create file {:?}! \n{}", path, e))
        }
    };

    match file.write_all(contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Error writing to file! \n{}", e))
        },
    }

    Ok(contents)
}

fn parse_args(args: Args) -> (PathBuf, PathBuf) {
    let mut in_path = Path::new("input.mlog").to_owned();
    let mut out_path = Path::new("output.mlog").to_owned();
    let mut is_in = false; let mut is_out = false;
    for arg in args {
        if arg == "--help" {
            println!("Specify a file to input with -i and -o for an output file. The default file names are input.mlog and output.mlog respectively.");
        }


        else if is_in {
            in_path = Path::new(&arg).to_owned();
            is_in = false;
        } else if arg == "-i" && is_out == false {
            is_in = true;
        }

        else if is_out {
            out_path = Path::new(&arg).to_owned();
            is_out = false;
        } else if arg == "-o" && is_in == false {
            is_out = true;
        }
    }

    if is_in || is_out {
        panic!("Improper arguments! Use --help for help");
    }

    (in_path, out_path)
}

fn main() {
    #[allow(non_snake_case)]
    let JUMP_REGEX: Regex = Regex::new(r"jump (\d+) (.+)").unwrap();

    let (in_path, out_path) = parse_args(env::args());

    let input = match read_file(&in_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file! \n{}", e);
            process::exit(1);
        },
    };

    let jump_to_index = read_jumps(&input, &JUMP_REGEX);
    let compiled = compile(&input, &jump_to_index, &JUMP_REGEX);

    let result = write_file(&out_path, compiled);
    if let Err(err) = result {
        eprintln!("{}", err);
    } else {
        println!("Successfully written to {:?}", &out_path);
    }
}

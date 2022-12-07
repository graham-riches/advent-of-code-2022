extern crate utilities;
pub mod fs;

// Parse result for an input line
#[derive(Debug, Clone, PartialEq)]
enum LineResult {
    Action(fs::Action),
    Directory(String),
    File(fs::File)
}

fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();

    // Build filesystem
    let mut fs = fs::FileSystem::new();    
    for l in lines.iter() {
        match parse_line(l) {
            LineResult::Action(a)         => match a {
                fs::Action::ListAll       => (),
                fs::Action::GotoRoot      => fs.go_to_root(),
                fs::Action::GoUpOne       => fs.go_up_one(),
                fs::Action::GoDownTo(dir) => fs.go_down_to(&dir),
            },
            LineResult::Directory(d) => fs.add_directory(&d),
            LineResult::File(f)      => fs.add_file(f),
        }
    }
    
    // Part one
    let dirs = fs.get_directories_with_size_below(100000);
    let total_size = dirs.iter()
     .fold(0, |sum, x| sum + x.size);
    println!("Part one: {}", total_size);

    // Part two
    let unused_size = 70000000 as usize - fs.get_used_size();
    let delete_at_least = 30000000 - unused_size;
    let dir_to_delete = fs.get_first_directory_bigger_than(delete_at_least);
    println!("{:?}", dir_to_delete);

}

// Parse a line into either a console command, or a output result
fn parse_line(s: &str) -> LineResult {
    let items: Vec<&str> = s.split(" ").collect();
    match items[0] {
        "$"   => LineResult::Action(parse_action(s)),
        "dir" => LineResult::Directory(items[1].to_string()),
        _     => LineResult::File(fs::File::new(items[1], items[0].parse::<usize>().unwrap()))
    }
}

// Parse a line into a filesystem action
fn parse_action(s: &str) -> fs::Action {
    let items: Vec<&str> = s.split(" ").collect();
    match items[1] {
        "ls" => fs::Action::ListAll,
        _    => match items[2] {
            "/"  => fs::Action::GotoRoot,
            ".." => fs::Action::GoUpOne,
            _    => fs::Action::GoDownTo(items[2].to_string())
        }
    }
}


#[test]
fn test_parse_action() {
    assert_eq!(parse_action("$ cd /"), fs::Action::GotoRoot);
    assert_eq!(parse_action("$ ls"), fs::Action::ListAll);
    assert_eq!(parse_action("$ cd .."), fs::Action::GoUpOne);
    assert_eq!(parse_action("$ cd d"), fs::Action::GoDownTo("d".to_string()));
}
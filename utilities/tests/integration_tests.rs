#[test]
fn test_read_lines_from_file() -> Result<(), std::io::Error> {
   let lines = utilities::lines_from_file("tests/data/lines.txt")?;
   assert_eq!(lines.len(), 10);
   Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(utilities::parse_pair::<i32>("1,2",","), Some((1, 2)));
}
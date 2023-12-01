pub mod common {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    pub fn read_lines<S: Into<String>>(filename: S) -> io::Lines<BufReader<File>> {
        // Open the file in read-only mode.
        let file = File::open(filename.into()).unwrap();
        // Read the file line by line, and return an iterator of the lines of the file.
        return BufReader::new(file).lines()
    }

    pub fn read_valid_lines<S: Into<String>>(filename: S) -> impl Iterator<Item=String> + 'static {
        // Open the file in read-only mode.
        let file = File::open(filename.into()).unwrap();
        // Read the file line by line, and return an iterator of the lines of the file.
        return BufReader::new(file).lines()
            .filter_map(|l| l.ok());
    }
}
extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = trace(contents.trim());
    println!("Part 1: {}", part1);
}

fn trace(contents: &str) -> &str {
    contents
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!("ABCDEF", trace(input));
    }
}

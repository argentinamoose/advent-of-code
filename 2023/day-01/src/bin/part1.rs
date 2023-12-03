fn main() {
   println!("part1");
   let input = include_str!("./input.txt");
   let output = part1(input);
   dbg!(output);
}

fn part1(input: &str) -> String {
   todo!()
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn it_works() {
      let result = part1("");
      assert_eq!(result, "4".to_str());
   }
}
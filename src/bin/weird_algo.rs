fn main() {
    // reading input from the user as a string and parsing it to u64 and unwrapping it 
    // read_to_string function can either be used with a file or a buffer
    let mut input:u64 =std::io::read_to_string(std::io::stdin())
   .unwrap()
    .trim()
    .parse()
    .unwrap();
    print!("{}", input);
    // while loop to check if the input is greater than 1
    // if the input is greater than 1, the loop will continue
    // if the input is less than 1, the loop will break
    // in the loop, we check if the input is even or odd
    // if the input is even, we divide the input by 2
    // if the input is odd, we multiply the input by 3 and add 1

   while input >1{
         print!("{} ", input);
         if input % 2 == 0 {
              input /= 2;
         } else {
              input = input * 3 + 1;
         }
         print!("{}", input);
   } 
}

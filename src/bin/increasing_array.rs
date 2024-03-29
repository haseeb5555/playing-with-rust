fn main(){
   let mut inp :Vec<u64> = std::io::read_to_string(std::io::stdin())
    .unwrap()
    .lines()
    .skip(1)
    .flat_map(|x| x.split_whitespace())
    .map(|x| x.parse().unwrap())
    .collect();
   let mut moves = 0;

   for i in 1..inp.len(){
    if inp[i]<inp[i-1]{
        moves+=inp[i-1]-inp[i];
        inp[i]=inp[i-1];
    }
   }
   print!("{}", moves)
} 
fn main(){
    let inp = std::io::read_to_string(std::io::stdin())
    .unwrap()
    .trim()
    .parse::<u64>()
    .unwrap();
    if inp==2 || inp ==3 {
        print!("NO SOLUTION");
    }else{
       for i in (2..inp).rev().step_by(2){
           print!("{} ", i);

       }
         for i in (1..inp+1).rev().step_by(2){
              print!("{} ", i);
         }
         
    }
    print!("")
}

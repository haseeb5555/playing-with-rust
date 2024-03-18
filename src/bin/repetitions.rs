fn main(){
    // reading input from the user as a string
    let  input =std::io::read_to_string(std::io::stdin()).unwrap();
    // creating a peekable iterator from the input
    // peekable is used to look at the next element in the iterator without consuming it  
    let mut dna = input.chars().peekable();
    let mut max =1;
    let mut count =1;
    // Some is used to check if the iterator has a value
    // dna.next() is used to get the next value from the iterator
    // dna.peek() is used to look at the next value from the iterator without consuming it
    // &next is used to get the value of the next element
    while let Some(c) = dna.next(){
        if let Some(&next) = dna.peek(){
            if c == next{
                count +=1;
                if count > max{
                    max = count;
                }
            }else{
                count =1;
            }
        }
    }

    println!("{}", max);
}

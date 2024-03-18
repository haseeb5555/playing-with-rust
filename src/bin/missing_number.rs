fn main() {
    // vec is used to store the input from the user of same type
    let mut buf = Vec::new();
    // for loop to read the 2 times input from the user and push into the vec
    for i in std::io::stdin().lines().take(2) {
        buf.push(i.unwrap());
    }
    
    // parse the first input to u64 and unwrap it
    let n: u64 = buf[0].parse().unwrap();
    println!("{}", n);
    // split the second input by whitespace and map it to u64 and sum it
    // map is used to convert the string to u64
    // sum is used to add the u64 numbers
    let sum: u64 = buf[1]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .sum();
    println!("{}", sum);
    let missing_number = n * (n + 1) / 2 - sum;
    println!("{}", missing_number);
}

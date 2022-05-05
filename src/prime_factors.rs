pub fn get_prime_factors(n: i32)-> Vec<i32>{
    println!("pass here");
    let mut vector: Vec<i32> = Vec::new();
    let mut i: i32 = n;
    let mut carry: i32 = 2;
    while(i > 1){
        if i % carry == 0{
            vector.push(carry);
            i = i / carry;
        }else{
            carry += 1;
        };
    }
    return vector;
}

pub fn get_sum(number: i32)->Vec<i32>{
    let mut tmp: i32 = number;
    let mut digits: Vec<i32> = Vec::new();
    while tmp > 0
    {
        digits.push(tmp%10);
        tmp /= 10;
    }
    reverse(digits)
}

//function for return revers of Vec<i32>
pub fn reverse(digits: Vec<i32>)->Vec<i32>{
    let mut tmp: Vec<i32> = Vec::new();
    for i in digits.iter()
    {
        tmp.push(*i);
    }
    tmp.reverse();
    tmp
}




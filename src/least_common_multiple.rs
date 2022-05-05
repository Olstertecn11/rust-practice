pub fn get_common_multiple(n1: i32, n2: i32, n3:i32) -> i32{
    let max = get_bigger_num(n1, n2, n3);
    let mut i: i32 = max;
    loop{
        if i % n1 == 0 && i % n2 == 0 && i % n3 == 0 {
            break i;
        }
        i += 1;
    }
}



fn get_bigger_num(n1: i32, n2: i32, n3:i32)->i32{
    let mut max: i32 = n1;
    if n2 > max{max = n2}
    if n3 > max{max = n3}
    return max;
}

pub fn is_pangram(text: String)->bool{
    let mut count: i32 = 0;
    for i in text.chars()
    {
        if exist_in(i){
            count += 1;
        }
    }
    count == text.len().try_into().unwrap()
}


fn exist_in(c: char)-> bool{
    let abc = String::from("abcdefghijklmnopqrstuvwxyz");
    for i in abc.chars(){
        if c == i{
            return true;
        }

    }
    false
}

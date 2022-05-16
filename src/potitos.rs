use std::{fs::File, io::{BufReader, BufRead}};

fn print_ingredients(arr: Vec<String>){
    println!("printing ingredients");
    let mut x: i32 = 0;
    for i in 0..arr.len(){
        let line: String = arr[i].to_string();
        if line.len() < 2{
            //convert line to integer
            x = line.parse::<i32>().unwrap();
            //convert usize i to int
            let  _i = i as i32;
            let _top: i32 = x + _i;
            println!("Internal loop: ");
            for _ in _i.._top{
                print!("{}", "");
            }
        }
        println!("{}. {}",x,arr[i]);
        x = x + 1;
    }
}



pub fn read_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ingredients: Vec<String> = Vec::new(); 
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        ingredients.push(line?);
    }
    print_ingredients(ingredients);
    Ok(())
}











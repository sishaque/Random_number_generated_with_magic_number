use rand::Rng;




fn main() {

    let mut rng = rand::thread_rng(); // declares rng as dynamic variable
   
    let num = magic_num::magic_number(); // defines num as magic number

    if num == 0 
    {
        println!("{} unsigned byte detected, thus smaller range!", num);
        // generates and prints unsigned number between 0 and 231
        let n2: u8 = rng.gen_range(0..=231); 
        println!("{} is a unsigned random number between 0 and 231", n2);
    }
    else 
    {
        println!("{} unsigned byte detected, thus normal range!", num);
        // generates and prints unsigned number between 0 and 255
        let n1: u8 = rng.gen();
        println!("{} is a unsigned random number between 0 and 255", n1);


    }


    
    
}


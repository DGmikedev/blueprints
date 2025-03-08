use rand::{self, Rng};

pub fn genera_token(lon: usize )->String{

    // Generador de strings seguros!
    let mut clve: String = String::from("");
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    
    let characters: [&str;67] =[
    "A","B","C","D","E","F","G","H","I","J",
    "K","L","M","N","O","P","Q","R","S","T",
    "U","V","W","X","Y","Z","a","b","c","d",
    "e","f","g","h","i","j","k","l","m","n",
    "o","p","q","r","s","t","u","v","w","x",
    "y","z","0","1","2","3","4","5","6","7",
    "8","9","!","@","#","$","%"
    ];
    
    for _i in 0..= lon {

        let indx = rng.random_range(0..=66);
        clve.push_str(characters[indx]);
    }
    
    clve
    
}
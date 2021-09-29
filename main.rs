use std::io;

/* CHECK TYPE OF VARIABLE */
/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
print_type_of(&user_input);
*/

fn main() { 
    println!("Player A vs Player B");
    println!("Enter Player A's ranking:");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read from stdin");
    
    let firsttrimmed = first.trim();
    let aint: i32 = firsttrimmed.trim().parse().ok().expect("Error! user entered value is not an integer");
    match firsttrimmed.parse::<u32>() {
        Ok(_i) => println!("Enter Player B's ranking:"),                
        Err(..) => println!("Error! user entered value is not an integer: {}", firsttrimmed),
    };  
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read from stdin");

    let secondtrimmed = second.trim();
    let bint: i32 = secondtrimmed.trim().parse().ok().expect("Error! user entered value is not an integer");
    match secondtrimmed.parse::<u32>() {
    Ok(_i) => {
        println!("Expected Score for Player A: {}", expected_player_a_prob(aint, bint));
        println!("Expected Score for Player A: {}", 1.0 - expected_player_a_prob(aint, bint));
        let updated_rating = |actual_score:f32| aint as f32 + (k_factor(aint)) as f32 *(actual_score - expected_player_a_prob(aint, bint));
        println!("Updated Ranking for Player A: {}", updated_rating(0.0) as i32);
    },
    Err(..) => println!("Error! user entered value is not an integer: {}", secondtrimmed),
    }; 
    
    
}

fn expected_player_a_prob(rating_a:i32, rating_b:i32) -> f32 {
    let rating_diff = rating_b - rating_a;
    let exponent:f32 = rating_diff as f32 / 400.0;
    let denominator:f32 = 1.0 + f32::powf(10.0, exponent);
    let expected_score:f32 = 1.0 / denominator;
    expected_score
}


fn k_factor(player_rating:i32) -> i8 {
    let mut k : i8 = 0;
    if player_rating < 2100 {
        k = 32;
    }
    else if player_rating > 2100 && player_rating < 2400 {
        k = 24;
    }
    else {
        k = 16;
    }
    k  
}

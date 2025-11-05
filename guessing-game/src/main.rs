use std::{io::{Write, stdin, stdout}};
use rand::Rng;

struct Card{
    card: String
}

fn generate_new_ascii_char_num()-> u32 {
    let first_number = 97;
    let last_number = first_number + 26;
    let mut rng = rand::thread_rng();
    rng.gen_range(first_number..last_number)
}   

fn generate_random_card(nums: &mut Vec<u32>)-> Card{
    let random_num:u32;
    
    loop{
        let new_num = generate_new_ascii_char_num();
        if !nums.contains(&new_num) {
            random_num = new_num;
            nums.push(random_num);
            break;
        }
    }
    let char = std::char::from_u32(random_num).unwrap();
    Card { card: char.to_string() }
}

fn generate_target_card(nums: Vec<u32>, card_count: usize )-> u32{
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..card_count);
    nums[index]
}

fn print_cards(cards: Vec<Card>){
    print!("Cards:");
    print!("|\t");
    for card in cards{
        print!("{}\t|\t", card.card);
    }
    println!();
}

fn get_input()-> u32{
    print!("Enter card name: ");
    let mut char_buf = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut char_buf).unwrap();
    let ch = match char_buf.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("No input detected defaulting ''");
            ' '
        }
    };
    println!("==============================================");
    stdout().flush().unwrap();
    ch as u32
}

fn show_result(win: u32, count: u32){
    let win = 2 * win;
    if win > count{
        println!("👑👑👑 you are win")
    }
    else if win == count{
        println!("😐😐😐 Match is tied")
    }
    else{
        println!("😥😥😥 you are lose")
    }


}

fn main() {
    let card_count = 3;

    println!("===== Guessing game =======");
    let mut guessing_lcount_str = String::new();
    print!("Enter loop count: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut guessing_lcount_str).unwrap();

    let count: u32 = guessing_lcount_str
        .trim()
        .parse()
        .unwrap_or_else(|_| {
            println!("Invalid count, using default 5");
            5
        });

    let mut win = 0;

    for _ in 0..count{
        let mut cards : Vec<Card> = Vec::new();
        let mut generated_nums :Vec<u32> = Vec::new();
        for _ in 0..card_count{
            cards.push(generate_random_card(&mut generated_nums));
        }
        print_cards(cards);
        let target_card = generate_target_card(generated_nums, card_count);
        let input_number = get_input();

        if target_card == input_number{
            win += 1;
        }
    }
    show_result(win, count);


}

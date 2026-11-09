use rand::seq::SliceRandom;
use rand::rng;
use std::io;

fn get_user_play() -> bool {
    println!("Keep Playing: 1\n Stop: 2");
    let mut input = String::new();
    let choice: bool;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input2: i32 = input.trim().parse().expect("Please type a valid number!");

    if input2 == 1 {
        choice = true;
    } else {
        choice = false;
    }
    choice
}

fn get_user_bet(chips_av: i32) -> i32 {
    loop{
        println!("Your Bet:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Please type a valid number!");
        if chips_av >= input{
            break;
        }
    }
    input
}

fn get_user_choice() -> bool {
    println!("Hit: 1\nStand: 2");
    let mut input = String::new();
    let choice: bool;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    let input2: i32 = input.trim().parse().expect("Please type a valid number!");

    if input2 == 1 {
        choice = true;
    } else {
        choice = false;
    }
    choice
}
fn print_card(value: i32) {
    let mut maybe_ace: String = value.to_string();
    if value == 11{
        maybe_ace = "A".to_string();
    }
    println!("┌─────────┐");
    println!("│ {:<8}│", maybe_ace);
    println!("│         │");
    println!("│         │");
    println!("│         │");
    println!("│{:>8} │", maybe_ace);
    println!("└─────────┘");
}

fn print_card_d() {
    println!("┌─────────┐");
    println!("│^^^^^^^^^│");
    println!("│vvvvvvvvv│");
    println!("│^^^^^^^^^│");
    println!("│vvvvvvvvv│");
    println!("│^^^^^^^^^│");
    println!("└─────────┘");
}


fn main() {

    let mut player_won: i32; // 1 is win 2 is loss 3 is push
    let mut player_chips: i32 =10_000;
    let mut bet: i32 = 0;
    let mut dealer_total: i32 = 0;
    let mut player_total: i32 = 0;
    let mut dealer_first: i32;
    let mut dealer_hidden: i32;
    let mut dealer_amount_aces: i32 = 0;
    let mut player_amount_aces: i32 = 0;
    let mut dealer_bust: bool = false;
    let mut hit: bool; 
    let mut bust: bool = false;
    let mut deck: Vec<i32> = Vec::new();
    let mut card: i32;
    for _ in 0..24 {
        deck.extend(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11]);
    }
    deck.shuffle(&mut rng());

    while get_user_play() {
        println!("Amount of chips: {player_chips}");
        bet = get_user_bet(player_chips);
        // dealer cards assign
        println!("----------\nDEALER CARDS");
        card = deck.pop().unwrap();
        if card == 11{
            dealer_amount_aces += 1
        }
        dealer_total += card;
        dealer_first = card;
        print_card(card);
        card = deck.pop().unwrap();
        if card == 11{
            dealer_amount_aces += 1
        }
        dealer_total += card; 
        dealer_hidden = card;
        print_card_d();
        if dealer_total > 21 && dealer_amount_aces >= 1{
            dealer_total -= 10;
            dealer_amount_aces -= 1;
        }

        //player cards
        println!("----------\nPLAYER CARDS");
        card = deck.pop().unwrap();
        if card == 11{
            player_amount_aces += 1
        }
        player_total += card;
        print_card(card);
        card = deck.pop().unwrap();
        if card == 11{
            player_amount_aces += 1
        }
        player_total += card;
        if player_total > 21 && player_amount_aces >= 1{
            player_total -= 10;
            player_amount_aces -= 1;
        }

        //hit player logic
        print_card(card); 
        loop{
            hit = get_user_choice();
            if hit{
                card = deck.pop().unwrap();
                if card == 11{
                    player_amount_aces += 1
                }
                player_total += card;
                print_card(card);
                if player_total > 21{
                    bust = true;
                    if player_amount_aces >= 1{
                        bust = false;
                        player_total -= 10;
                        player_amount_aces -= 1;
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                break;
            }
        }

        // dealer continued logic
        if !bust {
            println!("----------\nDEALER CARDS CONTINUED");
            println!("the dealers cards were:");
            print_card(dealer_first);
            print_card(dealer_hidden);
            while dealer_total <= 16 {
                println!("dealer hits on {dealer_total}");
                card = deck.pop().unwrap();
                if card == 11{
                    dealer_amount_aces += 1
                }
                dealer_total += card;
                print_card(card);
                if dealer_total > 21{
                    dealer_bust = true;
                    if dealer_amount_aces >= 1{
                        dealer_bust = false;
                        dealer_total -= 10;
                        dealer_amount_aces -= 1;
                    }
                }
            }
        }

        //win logic
        if bust && !dealer_bust{
            println!("BUST");
            player_won = 2;
        }
        else if player_total > dealer_total {
            println!("YOU WON");
            player_won = 1;
        }
        else if dealer_bust{
            println!("YOU WON DEALER IS BUST");
            player_won = 1;
        }
        else if player_total == dealer_total {
            println!("PUSH");
            player_won = 3;
        }
        else{
            println!("YOU LOST");
            player_won = 2;
        }
        

        // chips assign
        if player_won == 1{
            player_chips += bet;
        }
        else if player_won == 2 {
            player_chips -= bet;
        }
    }
}
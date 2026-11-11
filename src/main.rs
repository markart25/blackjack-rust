// written by mark (markart25)
// github.com/markart25/blackjack-rust
// markart.dev


use rand::seq::SliceRandom;
use rand::rng;
use std::io;

//all the functions

fn welcome(){
    println!(r#"
           /$$ /$$                                                            
          | $$|__/                                                            
  /$$$$$$$| $$ /$$                                                            
 /$$_____/| $$| $$                                                            
| $$      | $$| $$                                                            
| $$      | $$| $$                                                            
|  $$$$$$$| $$| $$                                                            
 \_______/|__/|__/                                                            
                                                                              
                                                                              
                                                                              
 /$$       /$$                     /$$                               /$$      
| $$      | $$                    | $$                              | $$      
| $$$$$$$ | $$  /$$$$$$   /$$$$$$$| $$   /$$ /$$  /$$$$$$   /$$$$$$$| $$   /$$
| $$__  $$| $$ |____  $$ /$$_____/| $$  /$$/|__/ |____  $$ /$$_____/| $$  /$$/
| $$  \ $$| $$  /$$$$$$$| $$      | $$$$$$/  /$$  /$$$$$$$| $$      | $$$$$$/ 
| $$  | $$| $$ /$$__  $$| $$      | $$_  $$ | $$ /$$__  $$| $$      | $$_  $$ 
| $$$$$$$/| $$|  $$$$$$$|  $$$$$$$| $$ \  $$| $$|  $$$$$$$|  $$$$$$$| $$ \  $$
|_______/ |__/ \_______/ \_______/|__/  \__/| $$ \_______/ \_______/|__/  \__/
                                       /$$  | $$                              
                                      |  $$$$$$/                              
                                       \______/                               
    

    Welcome to CLI blakjack written in rust.
    - the rules are pretty simple as long as you know the blackjack rules. if you dont please refer to https://www.wikihow.com/Play-Blackjack or similar to learn the game
    - this is a one player game against a dealer 
    "#)
}


fn get_user_play(chips: i32) -> bool {
    println!("Amount of chips: {chips}");
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
    let mut bet: i32;
    loop{
        println!("Your Bet:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        bet = input.trim().parse().expect("Please type a valid number!");
        if chips_av >= bet && bet > 0 {
            break;
        }
        else {
            println!("You do not have the chips to bet {bet}\nYou have {chips_av} available")
        }
    }
    bet
}

fn get_user_choice() -> i32 {
    println!("Hit: 1\nStand: 2");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: i32 = input.trim().parse().expect("Please type a valid number!");
    choice
}


fn get_user_first_choice(first: i32, second: i32) -> i32 {
    if first == second{
        println!("Hit: 1\nStand: 2\nDouble Down: 3\nSplit: 4");
    }
    else {
        println!("Hit: 1\nStand: 2\nDouble Down: 3");
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: i32 = input.trim().parse().expect("Please type a valid number!");
    choice
}

fn split(playerscard: i32, deck: &mut Vec<i32>, player_total_1: &mut i32, player_total_2: &mut i32,
player_amount_aces_1: &mut i32, player_amount_aces_2: &mut i32, bust1: &mut bool, bust2: &mut bool) {
    *player_total_1 = playerscard;
    *player_total_2 = playerscard;
    if playerscard == 11{
        *player_amount_aces_1 = 1;
        *player_amount_aces_2 = 1;
    }
    let mut hit: i32;
    let mut card: i32;
    card = deck.pop().unwrap();
    if card == 11{
    *player_amount_aces_1 += 1
    }
    *player_total_1 += card;
    print_card(card);
    if *player_total_1 > 21{
        *bust1 = true;
        if *player_amount_aces_1 >= 1{
            *bust1 = false;
            *player_total_1 -= 10;
            *player_amount_aces_1 -= 1;
        }
    }   
    loop{
        hit = get_user_choice();
        if hit == 1{
            card = deck.pop().unwrap();
            if card == 11{
                *player_amount_aces_1 += 1
            }
            *player_total_1 += card;
            print_card(card);
            if *player_total_1 > 21{
                *bust1 = true;
                if *player_amount_aces_1 >= 1{
                    *bust1 = false;
                    *player_total_1 -= 10;
                    *player_amount_aces_1 -= 1;
                    }
                else {
                    break;
                }
            }
        }
        else{
            break;
        }
    }
    println!("----------\nNEXT HAND");
    card = deck.pop().unwrap();
    if card == 11{
    *player_amount_aces_2 += 1
    }
    *player_total_2 += card;
    print_card(card);
    if *player_total_2 > 21{
        *bust2 = true;
        if *player_amount_aces_2 >= 1{
            *bust2 = false;
            *player_total_2 -= 10;
            *player_amount_aces_2 -= 1;
        }
    }
    loop{
        hit = get_user_choice();
        if hit == 1{
            card = deck.pop().unwrap();
            if card == 11{
                *player_amount_aces_2 += 1
            }
            *player_total_2 += card;
            print_card(card);
            if *player_total_2 > 21{
                *bust2 = true;
                if *player_amount_aces_2 >= 1{
                    *bust2 = false;
                    *player_total_2 -= 10;
                    *player_amount_aces_2 -= 1;
                }
                else {
                    break;
                }
            }
        }
        else{
            break;
        }
    }


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
    println!("│/////////│");
    println!("│/////////│");
    println!("│/////////│");
    println!("│/////////│");
    println!("│/////////│");
    println!("└─────────┘");
}


// main ig

fn main() {
    let mut players_first_choice: bool;
    let mut player_blackjack: bool;
    let mut dealer_blackjack: bool;
    let mut player_won: i32; // 1 is win 2 is loss 3 is push 4 pays 3:2 
    let mut player_won_2: i32; 
    let mut player_chips: i32 =10_000;
    let mut bet: i32;
    let mut dealer_total: i32;
    let mut player_total: i32;
    let mut player_total_2: i32;
    let mut dealer_first: i32;
    let mut players_first: i32;
    let mut players_second: i32;
    let mut player_split: bool;
    let mut dealer_hidden: i32;
    let mut dealer_amount_aces: i32;
    let mut player_amount_aces: i32;
    let mut player_amount_aces_2: i32;
    let mut dealer_bust: bool;
    let mut hit: i32; 
    let mut bust: bool;
    let mut bust2: bool;
    let mut deck: Vec<i32> = Vec::new();
    let mut card: i32;

    welcome();

    for _ in 0..24 {
        deck.extend(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11]);
    }
    deck.shuffle(&mut rng());

    while player_chips > 0 && get_user_play(player_chips) {
        if deck.len() < 15 {
            println!("Reshuffling deck...");
            deck.clear();
            for _ in 0..24 {
                deck.extend(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11]);
            }
        }
        deck.shuffle(&mut rng());
        println!("Cards remaining: {}", deck.len());
        dealer_total = 0;
        player_total = 0;
        dealer_amount_aces = 0;
        player_amount_aces = 0;
        dealer_bust = false;
        bust = false;
        players_first_choice = true;
        player_total_2 = 0;
        player_amount_aces_2 = 0;
        bust2 = false;
        player_split = false;
        player_blackjack = false;
        dealer_blackjack = false;
        player_won = 0;
        player_won_2 = 0;
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



        //some ace and natural logic
        if dealer_total > 21 && dealer_amount_aces >= 1{
            dealer_total -= 10;
            dealer_amount_aces -= 1;
        }
        else if dealer_total == 21 {
            //natural
            dealer_blackjack = true;
        }
        else{
            dealer_blackjack = false;
        }
        
        //player cards
        println!("----------\nPLAYER CARDS");
        card = deck.pop().unwrap();
        if card == 11{
            player_amount_aces += 1
        }
        player_total += card;
        players_first = card;
        print_card(card);


        card = deck.pop().unwrap();
        player_total += card;
        players_second = card;

        //some ace and natural logic
        if card == 11{
            player_amount_aces += 1
        }
        if player_total > 21 && player_amount_aces >= 1{
            player_total -= 10;
            player_amount_aces -= 1;
        }
        else if player_total == 21 {
            //natural 
            player_blackjack = true;
        }
        else {
            player_blackjack = false;
        }


        //hit player logic
        print_card(card); 
        // player hit loop skips if dealers peeks and its natural
        if !dealer_blackjack{
            loop{
                //needs to be able to double down if first choice
                hit = if players_first_choice && player_chips >= bet * 2{
                    get_user_first_choice(players_first, players_second)
                }
                else {
                    get_user_choice()
                };
                if hit == 1{
                    player_blackjack = false; // if they hit it wont be a natural 
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
                else if hit == 3 && players_first_choice && player_chips >= bet * 2{ 
                    bet += bet;
                    // same logic as a hit but forced break at the end. could be made a function but cba lowk
                    player_blackjack = false; // if they hit it wont be a natural 
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
                    }
                    break;
                }
                else if hit == 4 && players_first == players_second && players_first_choice && player_chips >= bet * 2{
                    // split logic 
                    player_split = true;
                    split(players_first, &mut deck, &mut player_total, &mut player_total_2,
&mut player_amount_aces, &mut player_amount_aces_2,&mut bust, &mut bust2);
                    break;
                }
                else {
                    break;
                }
                players_first_choice = false;
            }
        }
        else{
            println!("DEALER HAS NATURAL BLACKJACK")
        }

        // dealer continued logic
        if !bust || (player_split && !bust2) {
            println!("----------\nDEALER CARDS CONTINUED");
            println!("the dealers cards were:");
            print_card(dealer_first);
            print_card(dealer_hidden);
            while (dealer_total <= 16) || (dealer_total == 17 && dealer_amount_aces >= 1) {
                dealer_blackjack = false; // if they hit it wont be a natural 
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
        if bust {
            println!("BUST");
            player_won = 2;
        }
        else if player_total > dealer_total {
            if player_blackjack{
                println!("YOU WON with natural");
                player_won = 4;
            }
            else {
                println!("YOU WON");
                player_won = 1;
            }
        }
        else if dealer_bust{
            if player_blackjack{
                println!("YOU WON with natural");
                player_won = 4;
            }
            else{
                println!("YOU WON DEALER IS BUST");
                player_won = 1;
            }
        }
        else if player_total == dealer_total {
            if player_blackjack && !dealer_blackjack{
                println!("YOU WON due to having natural");
                player_won = 4;
            }
            else if dealer_blackjack && !player_blackjack {
                println!("YOU LOST due to dealer having natural");
                player_won = 2;
            }
            else{
                println!("PUSH");
                player_won = 3;
            }
        }
        else{
            println!("YOU LOST");
            player_won = 2;
        }

        if player_split{
            if bust2 {
                println!("BUST");
                player_won_2 = 2;
            }
            else if player_total_2 > dealer_total {
                println!("YOU WON");
                player_won_2 = 1;
            }
            else if dealer_bust{
                println!("YOU WON DEALER IS BUST");
                player_won_2 = 1;
            }
            else if player_total_2 == dealer_total {
                if dealer_blackjack{
                    println!("YOU LOST due to dealer having natural");
                    player_won_2 = 2;
                }
                else{
                    println!("PUSH");
                    player_won_2 = 3; 
                }
            }
            else{
                println!("YOU LOST");
                player_won_2 = 2;
            }
        
        }
        

        // chips assign
        if player_won == 1{
            player_chips += bet;
        }
        else if player_won == 2 {
            player_chips -= bet;
        }
        else if player_won == 4 {
            player_chips += bet * 3 / 2;
        }

        if player_split{
            if player_won_2 == 1{
                player_chips += bet;
            }
        else if player_won_2 == 2 {
                player_chips -= bet;
            }
        }
    }
    if player_chips == 0{
        println!("You ran out of chips")
    }
}
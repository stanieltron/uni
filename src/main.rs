use std::io::stdin;
use uni;
use uni::Market;
use uni::User;

fn main() {
    println!("Hello, world!");

    let precision: f64 = 1.0;
    let precision_percentage: f64 = 1000.0;
    let fee: f64 = 0.003;

    let mut users: Vec<User> = Vec::new();

    println!("initialize market market ");

   
    let mut market = Market::init_from_userinput();
    User::new_from_userinput( &mut users);

    // User::new(String::from("adam"), &mut users);
    // let mut market = Market::init(500.0, 10.0, 500.0, 10.0);
   
    

    for i in 0..1000 {
        println!("co ideme teraz?");
        println!("enter 1: create new user");
        println!("enter 2: mint_ETH_X -  user mintne liquidity market");
        println!("enter 3: swap_ETH_for_X - user swapne svoj ETH za X z burzy");
        println!("enter 4: swap_X_for_ETH - user swapne svoj X za ETH z burzy");
        println!("enter 5: swap_ETH_for_Y - user swapne svoj ETH za Y z burzy");
        println!("enter 6: swap_Y_for_ETH - user swapne svoj Y za ETH z burzy");
        println!("enter 7: swap_X_for_Y - user swapne svoj X za ETH Y burzy");
        println!("enter 8: swap_Y_for_Y - user swapne svoj Y za ETH X burzy");

        let mut x = 0;
        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("failed to read from stdin");

        let trimmed = choice.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => x = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        match x {
            1 => User::new_from_userinput(&mut users),
            2 => User::mint_lex_from_userinput(&mut users, &mut market),
            3 => User::sell_e_for_x_from_userinput(&mut users, &mut market, fee),
            4 => User::sell_x_for_e_from_userinput(&mut users, &mut market, fee),
            5 => User::sell_e_for_y_from_userinput(&mut users, &mut market, fee),
            6 => User::sell_y_for_e_from_userinput(&mut users, &mut market, fee),
            7 => User::sell_x_for_y_from_userinput(&mut users, &mut market, fee),
            8 => User::sell_y_for_x_from_userinput(&mut users, &mut market, fee),

            _ => println!("nevymyslaj blbosti"),
        };

        println!("------------------------------------------------------");
        println!(
            "POOL ETH-X    ETH: {}    X: {}",
            market.pool_lex.0, market.pool_lex.1
        );
        println!(
            "POOL ETH-Y    ETH: {}    Y: {}",
            market.pool_ley.0, market.pool_ley.1
        );

        let num_of_users = users.len();

        for i in 0..num_of_users {
            println!("-----------------------");
            println!("ucet {}", users[i].username);
            println!("ETH: {}", users[i].owned_e);
            println!("X: {}", users[i].owned_x);
            println!("Y: {}", users[i].owned_y);
            println!("share in ETH-X liquidity pool {} %", users[i].owned_lex);
            println!("share in ETH-Y liquidity pool {}", users[i].owned_ley);
        }
        println!("------------------------------------------------------");
        println!();
    }
}

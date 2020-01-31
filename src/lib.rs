use std::io::stdin;

pub struct Market {
    pub pool_lex: (f64, f64),
    pub pool_ley: (f64, f64),
}

impl Market {
    pub fn init(exe: f64, exx: f64, eye: f64, eyy: f64) -> Market {
        Market {
            pool_lex: (exe, exx),
            pool_ley: (eye, eyy),
        }
    }

    pub fn init_from_userinput() -> Market {
        let mut input_exe = String::new();
        let mut input_exx = String::new();
        let mut input_eye = String::new();
        let mut input_eyy = String::new();
        let mut exe: u64 = 0;
        let mut exx: u64 = 0;
        let mut eye: u64 = 0;
        let mut eyy: u64 = 0;

        println!("enter num of ETH to initialize ETH-X market with (int):");
        stdin()
            .read_line(&mut input_exe)
            .expect("failed to read from stdin");

        let mut trimmed = input_exe.trim();
        match trimmed.parse::<u64>() {
            Ok(i) => exe = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        println!("enter num of X to initialize ETH-X market with (int):");
        stdin()
            .read_line(&mut input_exx)
            .expect("failed to read from stdin");

        trimmed = input_exx.trim();
        match trimmed.parse::<u64>() {
            Ok(i) => exx = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        println!("enter num of ETH to initialize ETH-Y market with (int):");
        stdin()
            .read_line(&mut input_eye)
            .expect("failed to read from stdin");

        trimmed = input_eye.trim();
        match trimmed.parse::<u64>() {
            Ok(i) => eye = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        println!("enter num of Y to initialize ETH-Y market with (int):");
        stdin()
            .read_line(&mut input_eyy)
            .expect("failed to read from stdin");

        trimmed = input_eyy.trim();
        match trimmed.parse::<u64>() {
            Ok(i) => eyy = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        Market {
            pool_lex: (exe as f64, exx as f64),
            pool_ley: (eye as f64, eyy as f64),
        }
    }
}

pub struct User {
    pub username: String,
    pub owned_e: f64,
    pub owned_x: f64,
    pub owned_y: f64,
    pub owned_lex: f64,
    pub owned_ley: f64,
}

impl User {
    pub fn new(username: String, users: &mut Vec<User>) {
        let user = User {
            username: username,
            owned_e: 1000.0,
            owned_x: 1000.0,
            owned_y: 1000.0,
            owned_lex: 0.0,
            owned_ley: 0.0,
        };

        users.push(user);
    }

    pub fn select_user(users: &mut Vec<User>) -> usize {
        println!("Choose user by number");
        let num_of_users = users.len();

        let mut user_number = 0;
        for i in 0..num_of_users {
            println!("{}  {}", i, users[i].username);
        }

        let mut user_n = String::new();
        stdin()
            .read_line(&mut user_n)
            .expect("failed to read from stdin");

        let num_of_users = users.len();
        let trimmed = user_n.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => {
                if i < 0 as usize || i > num_of_users as usize {
                    println!("zle cislo usera si zadal ty fetak, vybera sa user user 0 ");
                } else {
                    user_number = i;
                }
            }
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        println!("Selected user {}", users[user_number].username);
        user_number
    }

    pub fn sell_e_for_x(&mut self, market: &mut Market, fee: f64, e_amount: f64) {
        // let sender = ensure_signed(origin)?;

        if self.owned_e < e_amount {
            //nejaky mesage a exit ?
        } else {
            let e = market.pool_lex.0;
            let x = market.pool_lex.1;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;

            self.owned_e -= de;
            self.owned_x += dx;
            market.pool_lex.0 = e1;
            market.pool_lex.1 = x1;
        }
    }

    pub fn mint_lex(&mut self, users: &mut Vec<User>, market: &mut Market, amount_lex: (f64, f64)) {
        if self.owned_e < amount_lex.0 {
            println!("not enought ETH");
        } else if self.owned_x < amount_lex.1 {
            println!("not enought X");
        } else {
            // let prev_exx = market.pool_lex.0 * self.owned_lex / 100.0;
            let prev_market_e = market.pool_lex.0;

            market.pool_lex.0 += amount_lex.0;
            market.pool_lex.1 += amount_lex.1;

            //pridat zmenu % z lex ostatnym uctom
            //pridat force na rovnake e a x
            let num_of_users = users.len();
            let new_owned_lex =
                (market.pool_lex.0 * self.owned_lex / 100.0 + amount_lex.0) / market.pool_lex.0;
            for i in 0..num_of_users {
                users[i].owned_lex = prev_market_e * users[i].owned_lex / 100.0 / market.pool_lex.0;
            }
            self.owned_lex = new_owned_lex;
            self.owned_e -= amount_lex.0;
            self.owned_x -= amount_lex.0;
        }
    }

    pub fn mint_ley(&mut self, users: &mut Vec<User>, market: &mut Market, amount_ley: (f64, f64)) {
        if self.owned_e < amount_ley.0 {
            println!("not enought ETH");
        } else if self.owned_y < amount_ley.1 {
            println!("not enought Y");
        } else {
            // let prev_exx = market.pool_lex.0 * self.owned_lex / 100.0;
            let prev_market_e = market.pool_ley.0;

            market.pool_ley.0 += amount_ley.0;
            market.pool_ley.1 += amount_ley.1;

            //pridat zmenu % z lex ostatnym uctom
            //pridat force na rovnake e a x
            let num_of_users = users.len();
            let new_owned_ley =
                (market.pool_ley.0 * self.owned_ley / 100.0 + amount_ley.0) / market.pool_ley.0;
            for i in 0..num_of_users {
                users[i].owned_ley = prev_market_e * users[i].owned_ley / 100.0 / market.pool_ley.0;
            }
            self.owned_ley = new_owned_ley;
            self.owned_e -= amount_ley.0;
            self.owned_y -= amount_ley.0;
        }
    }

    pub fn new_from_userinput(users: &mut Vec<User>) {
        let mut username = String::new();
        println!("enter username:");
        stdin()
            .read_line(&mut username)
            .expect("failed to read from stdin");

        let user = User {
            username: username,
            owned_e: 1000.0,
            owned_x: 1000.0,
            owned_y: 1000.0,
            owned_lex: 0.0,
            owned_ley: 0.0,
        };

        users.push(user);
    }

    pub fn mint_lex_from_userinput(users: &mut Vec<User>, market: &mut Market) {
        let user_number = User::select_user(users);
        let mut e: f64 = 0.0;

        let k: f64 = market.pool_lex.1 / market.pool_lex.0;

        let mut input_eth = String::new();

        println!("enter num of ETH to mint liquidity pool ETH-X (float):");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        let x = k * e;
        let amount_lex: (f64, f64) = (e, x);

        if users[user_number].owned_e < amount_lex.0 {
            println!("not enought ETH");
        } else if users[user_number].owned_x < amount_lex.1 {
            println!("not enought X, neede {}", x);
        } else {
            // let prev_exx = market.pool_lex.0 * self.owned_lex / 100.0;
            let prev_market_e = market.pool_lex.0;

            market.pool_lex.0 += amount_lex.0;
            market.pool_lex.1 += amount_lex.1;

            //pridat zmenu % z lex ostatnym uctom
            //pridat force na rovnake e a x
            let num_of_users = users.len();

            for i in 0..num_of_users {
                if i == user_number {
                } else {
                    users[i].owned_lex = prev_market_e * users[i].owned_lex / market.pool_lex.0;
                }
            }
            users[user_number].owned_lex = (prev_market_e * users[user_number].owned_lex / 100.0
                + amount_lex.0)
                / market.pool_lex.0
                * 100.0;
            users[user_number].owned_e -= amount_lex.0;
            users[user_number].owned_x -= amount_lex.0;

            println!("{} added {} of ETH and {} of X to liquidity pool ETH-X, and now owns {} % of the pool", users[user_number].username, e, x, users[user_number].owned_lex);
        }

        //pridat nech zobrazi kolko x od neho pyta a ci je stym ok
    }

    pub fn mint_ley_from_userinput(users: &mut Vec<User>, market: &mut Market) {
        let user_number = User::select_user(users);
        let mut e: f64 = 0.0;

        let k: f64 = market.pool_ley.1 / market.pool_ley.0;

        let mut input_eth = String::new();

        println!("enter num of ETH to mint liquidity pool ETH-X (float):");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        let x = k * e;
        let amount_ley: (f64, f64) = (e, x);

        if users[user_number].owned_e < amount_ley.0 {
            println!("not enought ETH");
        } else if users[user_number].owned_x < amount_ley.1 {
            println!("not enought X, neede {}", x);
        } else {
            // let prev_exx = market.pool_ley.0 * self.owned_ley / 100.0;
            let prev_market_e = market.pool_ley.0;

            market.pool_ley.0 += amount_ley.0;
            market.pool_ley.1 += amount_ley.1;

            //pridat zmenu % z ley ostatnym uctom
            //pridat force na rovnake e a x
            let num_of_users = users.len();

            for i in 0..num_of_users {
                if i == user_number {
                } else {
                    users[i].owned_ley = prev_market_e * users[i].owned_ley / market.pool_ley.0;
                }
            }
            users[user_number].owned_ley = (prev_market_e * users[user_number].owned_ley / 100.0
                + amount_ley.0)
                / market.pool_ley.0
                * 100.0;
            users[user_number].owned_e -= amount_ley.0;
            users[user_number].owned_y -= amount_ley.0;

            println!("{} added {} of ETH and {} of X to liquidity pool ETH-X, and now owns {} % of the pool", users[user_number].username, e, x, users[user_number].owned_ley);
        }

        //pridat nech zobrazi kolko x od neho pyta a ci je stym ok
    }

    pub fn sell_e_for_x_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();

        println!("kolko ETH chces vymenit za X:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought ETH");
        } else {
            let e = market.pool_lex.0;
            let x = market.pool_lex.1;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;

            users[user_number].owned_e -= de;
            users[user_number].owned_x += dx;
            market.pool_lex.0 = e1;
            market.pool_lex.1 = x1;
            println!(
                "{} vymenil {} ETH za {} X",
                users[user_number].username, de, dx
            )
        }
    }

    //premenne sa volaju zle, pre rychlost, som len poprehadzoval hodnoty vo vnutri
    pub fn sell_e_for_y_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();

        println!("kolko ETH chces vymenit za Y:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought ETH");
        } else {
            let e = market.pool_ley.0;
            let x = market.pool_ley.1;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;

            users[user_number].owned_e -= de;
            users[user_number].owned_y += dx;
            market.pool_ley.0 = e1;
            market.pool_ley.1 = x1;
            println!(
                "{} vymenil {} ETH za {} Y",
                users[user_number].username, de, dx
            )
        }
    }

    //premenne sa volaju zle, pre rychlost, som len poprehadzoval hodnoty vo vnutri
    pub fn sell_x_for_e_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();

        println!("kolko X chces vymenit za ETH:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought X");
        } else {
            let e = market.pool_lex.1;
            let x = market.pool_lex.0;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;

            users[user_number].owned_e += dx;
            users[user_number].owned_x -= de;
            market.pool_lex.1 = e1;
            market.pool_lex.0 = x1;
            println!(
                "{} vymenil {} X za {} Y",
                users[user_number].username, de, dx
            )
        }
    }

    //premenne sa volaju zle, pre rychlost, som len poprehadzoval hodnoty vo vnutri
    pub fn sell_y_for_e_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();

        println!("kolko Y chces vymenit za ETH:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought Y");
        } else {
            let e = market.pool_ley.1;
            let x = market.pool_ley.0;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;

            users[user_number].owned_e += dx;
            users[user_number].owned_y -= de;
            market.pool_ley.1 = e1;
            market.pool_ley.0 = x1;
            println!(
                "{} vymenil {} Y za {} ETH",
                users[user_number].username, de, dx
            )
        }
    }

    //premenne sa volaju zle, pre rychlost, som len poprehadzoval hodnoty vo vnutri
    pub fn sell_x_for_y_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();
        let mut e_to_y = 0.0;
        println!("kolko X chces vymenit za Y:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought X");
        } else {
            let e = market.pool_lex.1;
            let x = market.pool_lex.0;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;
            e_to_y = dx;
            users[user_number].owned_e += dx;
            users[user_number].owned_x -= de;
            market.pool_lex.1 = e1;
            market.pool_lex.0 = x1;
            println!(
                "{} vymenil {} X za {} ETH",
                users[user_number].username, de, dx
            );
            println!("----------- STAV PO X - > ETH -----------------");
            println!(
                "POOL ETH-X    ETH: {}    X: {}",
                market.pool_lex.0, market.pool_lex.1
            );
            println!(
                "POOL ETH-Y    ETH: {}    Y: {}",
                market.pool_ley.0, market.pool_ley.1
            );
        }

        let e = market.pool_ley.0;
        let x = market.pool_ley.1;
        let g = 1.0 - fee;
        let de = e_to_y;
        let e1 = e + e_to_y; //pool
        let x1 = x * e / (e + de * g);
        let dx = x - x1;

        users[user_number].owned_e -= de;
        users[user_number].owned_y += dx;
        market.pool_ley.0 = e1;
        market.pool_ley.1 = x1;
        println!(
            "{} vymenil {} ETH za {} Y",
            users[user_number].username, de, dx
        );
        println!(
            "{} vymenil {} X za {} Y",
            users[user_number].username, e_amount, dx
        );
    }

    pub fn sell_y_for_x_from_userinput(users: &mut Vec<User>, market: &mut Market, fee: f64) {
        // let sender = ensure_signed(origin)?;
        let user_number = User::select_user(users);
        let mut e_amount = 0.0;
        let mut input_eth = String::new();
        let mut e_to_y = 0.0;
        println!("kolko Y chces vymenit za X:");
        stdin()
            .read_line(&mut input_eth)
            .expect("failed to read from stdin");

        let trimmed = input_eth.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => e_amount = i,
            Err(..) => println!("this was not an float: {}", trimmed),
        };

        if users[user_number].owned_e < e_amount {
            println!("not enought Y");
        } else {
            let e = market.pool_ley.1;
            let x = market.pool_ley.0;
            let g = 1.0 - fee;
            let de = e_amount;
            let e1 = e + e_amount; //pool
            let x1 = x * e / (e + de * g);
            let dx = x - x1;
            e_to_y = dx;
            users[user_number].owned_e += dx;
            users[user_number].owned_y -= de;
            market.pool_ley.1 = e1;
            market.pool_ley.0 = x1;
            println!(
                "{} vymenil {} Y za {} ETH",
                users[user_number].username, de, dx
            );
            println!("----------- STAV PO Y - > ETH -----------------");
            println!(
                "POOL ETH-X    ETH: {}    X: {}",
                market.pool_lex.0, market.pool_lex.1
            );
            println!(
                "POOL ETH-Y    ETH: {}    Y: {}",
                market.pool_ley.0, market.pool_ley.1
            );
        }

        let e = market.pool_lex.0;
        let x = market.pool_lex.1;
        let g = 1.0 - fee;
        let de = e_to_y;
        let e1 = e + e_to_y; //pool
        let x1 = x * e / (e + de * g);
        let dx = x - x1;

        users[user_number].owned_e -= de;
        users[user_number].owned_x += dx;
        market.pool_lex.0 = e1;
        market.pool_lex.1 = x1;

        println!(
            "{} vymenil {} ETH za {} X",
            users[user_number].username, de, dx
        );
        println!(
            "{} vymenil {} Y za {} X",
            users[user_number].username, e_amount, dx
        );
    }
}

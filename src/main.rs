use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg  in args {
        println!("{0}", arg);
    }
    println!("Hello, world!");
}
/*
Idea for CLI tool:
- NHL tracker?
- <base> schedule
    - Bring up the games that are being played over the next 5 days
- <base> schedule <team_abbrev>
    - Bring up the next 5 games that team is playing in
- <base> standings
    - Bring up the nhl standings (sorted in asc. order)


 */



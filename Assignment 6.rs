use std::io;
use rand::Rng;

struct Player {
    playerID: u8,
    name: String,
    score: u8,
    finalScore: u8
}
 
impl Player{
    fn new(playerName:String,playerID:u8) -> Player{
        Player {
            playerID : playerID,
            name: playerName,
            score: 0,
            finalScore: 0
        }
    }
}

    fn dice_roll() -> u8{
        let dice_num:u8 = rand::thread_rng().gen_range(1, 19);
        match dice_num{
            18 => 0,
            _ => dice_num,
        }  
            
        
    }



fn main() {
let mut Win = false;
let mut dice:Vec<Player> = Vec::new();
let mut temp = String::new();

println!("Enter Number of Players");
io::stdin().read_line(&mut temp).expect("Numeric Only");

let no_of_players: u8;
no_of_players = temp.trim().parse::<u8>().unwrap();
let mut player_object:Player;
for x in 1..=no_of_players {
    let mut temp2 = String::new();
    println!("Enter Player's {:?} Name: ?",x);
    let player_name: String;
    io::stdin().read_line(&mut temp2).expect("Character Only");
    player_name = temp2.trim().parse::<String>().unwrap();

    player_object = Player::new(player_name,x);
    dice.push(player_object);
}
println!("Hit Enter to Start Play");
println!("To Quit game press any other key");
println!("The New Team For The Match");

for p1 in 0..=dice.len()-1
{
    println!("Player # {} - {}",dice[p1].playerID,dice[p1].name);
}
let mut turnNo:u8 = 1;
while true{  
    let mut Key = String::new();  
    io::stdin().read_line(&mut Key).expect("operation failed");
   
    for j in 0..=dice.len()-1
    {
        let draw = dice_roll();
        dice[j].score = draw;
        if dice[j].finalScore + dice[j].score <= 99
        {
            
           let defeatingValue = dice[j].finalScore + dice[j].score;
           for find in 0..=dice.len()-1
           {
               if dice[find].finalScore == defeatingValue && find != j
                {
                    dice[find].finalScore = 0; 
                    println!("Alas! {} has kicked by {} at score of {}",dice[find].name,dice[j].name,defeatingValue);
                    break;
                }
           }
           dice[j].finalScore = defeatingValue;
        }
        if dice[j].finalScore + dice[j].score == 100
           {
                dice[j].finalScore = 100;
                println!("Congratz! Player {} has won on turn {}",dice[j].name,turnNo);
                Win = true;

                break;
            }
    }
    for print in 0..=dice.len()-1{
        if Win
           { break; }
    println!("Turn: {} Dice Roll of Player {} - {} is {} and total {}",turnNo,dice[print].playerID,dice[print].name,dice[print].score,dice[print].finalScore);
        
    }
    turnNo = turnNo + 1; 
   
     
    if Key != "\r\n" || Win
        

           { break; }
        
}

}

 
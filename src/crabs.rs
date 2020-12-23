use std::collections::HashSet;

pub struct Game {
    player1 : Vec<usize>,
    player2 : Vec<usize>
}

impl Game {
    pub fn from_lines<'a,I>(lines1 : I, lines2 : I) -> Game 
      where I : Iterator<Item = &'a str>
    {
        Game {
            player1 : lines1.filter_map(
                |item| item.parse::<usize>().ok()
            ).collect(),
            player2 : lines2.filter_map(
                |item| item.parse::<usize>().ok()
            ).collect()
        }
    }

    pub fn from_players(player1 : &Vec<usize>, num1 : usize, player2 : &Vec<usize>, num2 : usize) -> Game 
    {
        Game {
            player1 : player1.iter().skip(player1.len() - num1).cloned().collect(),
            player2 : player2.iter().skip(player2.len() - num2).cloned().collect(),
        }
    }

    pub fn play(self : &mut Self) {
        while !self.player1.is_empty() && !self.player2.is_empty() {
            let card1 = self.player1.pop().unwrap();
            let card2 = self.player2.pop().unwrap();
            if card1 > card2 {
                self.player1.insert(0, card1);
                self.player1.insert(0, card2);
            } else {
                self.player2.insert(0, card2);
                self.player2.insert(0, card1);
            }
        }
    }

    pub fn player1win(self : &Self) -> bool {
        self.player1.len() !=0
    }

    pub fn play_recursive(self : &mut Self) -> bool {
        let mut previous : HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
        while !self.player1.is_empty() && !self.player2.is_empty() {
            if previous.contains( &(self.player1.clone(), self.player2.clone()) ) {
                return true;
            }
            previous.insert( (self.player1.clone(), self.player2.clone()) );
            let card1 = self.player1.pop().unwrap();
            let card2 = self.player2.pop().unwrap();
            let mut player1win = card1 > card2;
            if card1 <= self.player1.len() && card2 <=  self.player2.len() {
                // Recurse
                let mut recursive_game = Game::from_players(
                    &self.player1,
                    card1, 
                    &self.player2,
                    card2
                );
                player1win = recursive_game.play_recursive();
            } 
            if player1win {
                self.player1.insert(0, card1);
                self.player1.insert(0, card2);
            } else {
                self.player2.insert(0, card2);
                self.player2.insert(0, card1);
            }  
        }
        return self.player1win();
    }

    pub fn score(self : &Self) -> usize {
        if !self.player1.is_empty() {
            self.player1.iter().zip(1..).map(
                |(a,b)| a*b
            ).sum()
        } else {
            self.player2.iter().zip(1..).map(
                |(a,b)| a*b
            ).sum()
        }
    }
}
fn shift_idx(len: usize, shift: i32) -> usize {
    if shift < 0 {
        let mut idx = shift;
        while idx < 0 {
            idx += len as i32
        }
        idx as usize
    } else {
        let mut idx = shift as usize;
        while idx >= len {
            idx -= len
        }
        idx as usize
    }
}

#[derive(Debug)]
struct Game {
    circle: Vec<usize>,
    current: usize, // the index of the current marble
    marble: usize,  // the number of the marble that was just played
    scores: Vec<usize>,
    current_player: usize,
}

impl Game {
    fn new(players: usize) -> Game {
        Game {
            circle: vec![0],
            current: 0,
            marble: 0,
            scores: vec![0; players],
            current_player: 0,
        }
    }

    fn next(&mut self) {
        self.marble += 1;
        if self.marble % 23 == 0 {
            self.scores[self.current_player] += self.marble;
            self.current = shift_idx(self.circle.len(), self.current as i32 - 7);
            self.scores[self.current_player] += self.circle.remove(self.current);
        } else {
            let idx = if self.current >= self.circle.len() - 1 {
                1
            } else {
                self.current + 2
            };
            self.circle.insert(idx, self.marble);
            self.current = idx;
        }
        self.current_player = shift_idx(self.scores.len(), self.current_player as i32 + 1);
    }
}

fn high_score(players: usize, last: usize) -> usize {
    let mut game = Game::new(players);

    while game.marble <= last {
        game.next();
    }
    *game.scores.iter().max().unwrap()
}

fn parse_input(s: &str) -> (usize, usize) {
    let tokens: Vec<_> = s.split_whitespace().collect();
    (tokens[0].parse().unwrap(), tokens[6].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_score() {
        let tests = [
            // Examples
            ("9 players; last marble is worth 25 points", 32),
            ("10 players; last marble is worth 1618 points", 8317),
            ("13 players; last marble is worth 7999 points", 146373),
            ("17 players; last marble is worth 1104 points", 2764),
            ("21 players; last marble is worth 6111 points", 54718),
            ("30 players; last marble is worth 5807 points", 37305),
            // Puzzle input
            ("403 players; last marble is worth 71920 points", 439089),
            (
                "403 players; last marble is worth 7192000 points",
                3668541094,
            ),
        ];
        for (input, score) in &tests {
            let (players, last) = parse_input(input);
            assert_eq!(high_score(players, last), *score);
        }
    }
}

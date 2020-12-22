use std::collections::HashSet;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines = input_data.lines().collect::<Vec<&str>>();
    let mut a_cards: Vec<u64> = Vec::new();
    let mut b_cards: Vec<u64> = Vec::new();

    let mut i = 1;
    while lines[i].len() > 0 {
        a_cards.push(lines[i].parse::<u64>().unwrap());
        i += 1;
    }
    i += 2;

    while i < lines.len() {
        b_cards.push(lines[i].parse::<u64>().unwrap());
        i += 1;
    }

    let now = Instant::now();
    match part {
        1 => solve_part1(a_cards, b_cards),
        2 => solve_part2(a_cards, b_cards),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(mut a_cards: Vec<u64>, mut b_cards: Vec<u64>) {
    while a_cards.len() > 0 && b_cards.len() > 0 {
        let a_card = a_cards.remove(0);
        let b_card = b_cards.remove(0);
        if a_card > b_card {
            a_cards.push(a_card);
            a_cards.push(b_card);
        } else {
            b_cards.push(b_card);
            b_cards.push(a_card);
        }
    }

    let winner_cards;
    if a_cards.len() > 0 {
        winner_cards = a_cards;
    } else {
        winner_cards = b_cards;
    }

    println!(
        "{}",
        winner_cards
            .iter()
            .rev()
            .enumerate()
            .fold(0, |x, y| x + (y.0 as u64 + 1) * y.1)
    );
}

fn play_game(a_cards_slice: &[u64], b_cards_slice: &[u64], root: bool) -> u64 {
    // This function will simulate a game
    // The root indicates if it is the first function called
    let mut a_cards: Vec<u64> = a_cards_slice.to_vec();
    let mut b_cards: Vec<u64> = b_cards_slice.to_vec();

    let mut winner = 0;
    let mut old_rounds: HashSet<(Vec<u64>, Vec<u64>)> = HashSet::new();
    while a_cards.len() > 0 && b_cards.len() > 0 {
        if old_rounds.contains(&(a_cards.clone(), b_cards.clone())) {
            winner = 0;
            break;
        } else {
            old_rounds.insert((a_cards.clone(), b_cards.clone()));
        }

        let a_card = a_cards.remove(0);
        let b_card = b_cards.remove(0);
        if a_card > a_cards.len() as u64 || b_card > b_cards.len() as u64 {
            if a_card > b_card {
                a_cards.push(a_card);
                a_cards.push(b_card);
            } else {
                b_cards.push(b_card);
                b_cards.push(a_card);
            }
        } else {
            let sub_game = play_game(
                &a_cards[0..a_card as usize],
                &b_cards[0..b_card as usize],
                false,
            );
            if sub_game == 0 {
                // a won the subgame
                a_cards.push(a_card);
                a_cards.push(b_card);
            } else if sub_game == 1 {
                // b won the subgame
                b_cards.push(b_card);
                b_cards.push(a_card);
            }
        }
    }

    if root {
        // Return the score of the winner
        let winner_cards;
        if a_cards.len() == 0 || b_cards.len() == 0 {
            if a_cards.len() > 0 {
                winner_cards = a_cards;
            } else {
                winner_cards = b_cards;
            }
        } else {
            if winner == 0 {
                winner_cards = a_cards;
            } else {
                winner_cards = b_cards;
            }
        }

        winner_cards
            .iter()
            .rev()
            .enumerate()
            .fold(0, |x, y| x + (y.0 as u64 + 1) * y.1)
    } else {
        // Return the winner of the game
        if a_cards.len() == 0 || b_cards.len() == 0 {
            if a_cards.len() > 0 {
                0
            } else {
                1
            }
        } else {
            winner
        }
    }
}

fn solve_part2(a_cards: Vec<u64>, b_cards: Vec<u64>) {
    println!("{}", play_game(&a_cards, &b_cards, true));
}

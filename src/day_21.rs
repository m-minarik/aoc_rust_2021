use std::cmp::max;

const NUM_PLAYERS: usize = 2;
const NUM_FIELDS: usize = 10;

const PART_1_POINTS: usize = 1000;
const PART_2_POINTS: usize = 21;
const PART_2_DICE_RESULTS: [usize; 27] = [3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9];

pub fn part_1(input: &str) -> usize {

    const SCORE_SEQUENCE: [[[usize; PART_1_POINTS]; NUM_FIELDS]; NUM_PLAYERS] = precompute_part_1();

    let (player_1_start, player_2_start): (usize, usize) = parse_input(input);

    let mut turn: usize = 0;

    while turn < PART_1_POINTS {
        // Player 1 wins
        if SCORE_SEQUENCE[0][player_1_start][turn] >= PART_1_POINTS {
            let num_rolls: usize = 6 * turn + 3;
            return num_rolls * SCORE_SEQUENCE[1][player_2_start][turn-1];
        } 

        // Player 2 wins
        if SCORE_SEQUENCE[1][player_2_start][turn] >= PART_1_POINTS {
            let num_rolls: usize = 6 * turn + 6;
            return num_rolls * SCORE_SEQUENCE[0][player_1_start][turn];
        }

        turn += 1;
    }

    0
}

pub fn part_2(input: &str) -> usize {

    const UNIVERSES_WITH_K_TURNS_TO_21: [[usize; PART_2_POINTS]; NUM_FIELDS] = precompute_part_2();

    let (player_1_start, player_2_start): (usize, usize) = parse_input(input);

    let player_1_games_ended: [usize; PART_2_POINTS] = UNIVERSES_WITH_K_TURNS_TO_21[player_1_start];
    let player_2_games_ended: [usize; PART_2_POINTS] = UNIVERSES_WITH_K_TURNS_TO_21[player_2_start];

    let mut player_1_games_continued: [usize; PART_2_POINTS] = [27; PART_2_POINTS];
    let mut player_2_games_continued: [usize; PART_2_POINTS] = [27; PART_2_POINTS];

    for i in 1..21 {
        player_1_games_continued[i] = player_1_games_continued[i-1] * 27 - player_1_games_ended[i]; 
        player_2_games_continued[i] = player_2_games_continued[i-1] * 27 - player_2_games_ended[i]; 
    }

    let mut player_1_wins: usize = 0;
    let mut player_2_wins: usize = 0;

    for i in 1..21 {
        player_1_wins += player_1_games_ended[i] * player_2_games_continued[i-1];
        player_2_wins += player_2_games_ended[i] * player_1_games_continued[i];
    }

    max(player_1_wins, player_2_wins)
    
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines: std::str::Lines<'_> = input.lines();

    let player_1_start: usize = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap() - 1;
    let player_2_start: usize = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap() - 1;

    (player_1_start, player_2_start)
}

const fn precompute_part_1() -> [[[usize; PART_1_POINTS]; NUM_FIELDS]; NUM_PLAYERS] {
    // Indexing
    let mut score_sequence: [[[usize; PART_1_POINTS]; NUM_FIELDS]; NUM_PLAYERS] = [[[0; PART_1_POINTS]; NUM_FIELDS]; NUM_PLAYERS];

    score_sequence[0][0] = _precompute_part_1::<1, 1>();
    score_sequence[0][1] = _precompute_part_1::<1, 2>();
    score_sequence[0][2] = _precompute_part_1::<1, 3>();
    score_sequence[0][3] = _precompute_part_1::<1, 4>();
    score_sequence[0][4] = _precompute_part_1::<1, 5>();
    score_sequence[0][5] = _precompute_part_1::<1, 6>();
    score_sequence[0][6] = _precompute_part_1::<1, 7>();
    score_sequence[0][7] = _precompute_part_1::<1, 8>();
    score_sequence[0][8] = _precompute_part_1::<1, 9>();
    score_sequence[0][9] = _precompute_part_1::<1, 10>();

    score_sequence[1][0] = _precompute_part_1::<2, 1>();
    score_sequence[1][1] = _precompute_part_1::<2, 2>();
    score_sequence[1][2] = _precompute_part_1::<2, 3>();
    score_sequence[1][3] = _precompute_part_1::<2, 4>();
    score_sequence[1][4] = _precompute_part_1::<2, 5>();
    score_sequence[1][5] = _precompute_part_1::<2, 6>();
    score_sequence[1][6] = _precompute_part_1::<2, 7>();
    score_sequence[1][7] = _precompute_part_1::<2, 8>();
    score_sequence[1][8] = _precompute_part_1::<2, 9>();
    score_sequence[1][9] = _precompute_part_1::<2, 10>();

    score_sequence
}

const fn _precompute_part_1<const PLAYER: usize, const START: usize>() -> [usize; PART_1_POINTS] {
    
    let mut score_sequence: [usize; PART_1_POINTS] = [0; PART_1_POINTS];
    let mut i: usize = START;
    let mut score: usize = 0;
    let mut turn: usize = 0;
    let mut steps: usize;
    while turn < PART_1_POINTS {
        if PLAYER == 1 {
            // let steps: usize = ((turn * 6 + 1) + (turn * 6 + 2) + (turn * 6 + 3));
            steps = 18 * turn + 6;
        } else {
            // let steps: usize = ((turn * 6 + 4) + (turn * 6 + 5) + (turn * 6 + 6));
            steps = 18 * turn + 15;
        }
    
        i = (i + steps - 1) % 10 + 1;
        score += i;
        score_sequence[turn] = score;
        turn += 1;
    }
    score_sequence
}

const fn precompute_part_2() -> [[usize; PART_2_POINTS]; NUM_FIELDS] {

    // Indexing: universes_with_k_turns_to_21[start_field_id][k]
    let mut universes_with_k_turns_to_21: [[usize; PART_2_POINTS]; NUM_FIELDS]  = [[0; PART_2_POINTS]; NUM_FIELDS];

    universes_with_k_turns_to_21[0] = _precompute_part_2::<1>();
    universes_with_k_turns_to_21[1] = _precompute_part_2::<2>();
    universes_with_k_turns_to_21[2] = _precompute_part_2::<3>();
    universes_with_k_turns_to_21[3] = _precompute_part_2::<4>();
    universes_with_k_turns_to_21[4] = _precompute_part_2::<5>();
    universes_with_k_turns_to_21[5] = _precompute_part_2::<6>();
    universes_with_k_turns_to_21[6] = _precompute_part_2::<7>();
    universes_with_k_turns_to_21[7] = _precompute_part_2::<8>();
    universes_with_k_turns_to_21[8] = _precompute_part_2::<9>();
    universes_with_k_turns_to_21[9] = _precompute_part_2::<10>();

    universes_with_k_turns_to_21
}

const fn _precompute_part_2<const START: usize>() -> [usize; PART_2_POINTS] {

    // field, points, turns
    let mut tmp: [[[usize; PART_2_POINTS+1]; PART_2_POINTS+1]; NUM_FIELDS] = [[[0; PART_2_POINTS+1]; PART_2_POINTS+1]; NUM_FIELDS];
    tmp[START-1][0][0] = 1;

    let mut turn: usize = 0;

    while turn < PART_2_POINTS - 1 {

        // Proceed by one turn
        let mut field: usize = 1;
        while field <= NUM_FIELDS {

            let mut points: usize = 0;
            while points < PART_2_POINTS {

                // Number of universes, where the player after `turn` turns has `points` points and stands on field `field`
                let mut res_id: usize = 0;
                while res_id < 27 {

                    let steps = PART_2_DICE_RESULTS[res_id];
                    let next_field = (field + steps - 1) % 10 + 1;
                    let mut next_points = points + next_field;
                    if next_points > PART_2_POINTS {
                        next_points = PART_2_POINTS;
                    }

                    tmp[next_field-1][next_points][turn + 1] += tmp[field-1][points][turn];

                    res_id += 1;
                }
                points += 1;
            }
            field += 1;
        }
        turn += 1;
    }

    // Vector of in how many universes player wins after K turns
    let mut universes_with_k_turns_to_21: [usize; PART_2_POINTS] = [0; PART_2_POINTS];
    
    // Reduce
    let mut turn: usize = 0;
    while turn < PART_2_POINTS{

        let mut field: usize = 0;
        while field < NUM_FIELDS {
            universes_with_k_turns_to_21[turn] += tmp[field][PART_2_POINTS][turn+1];
            field += 1;
        }
        turn += 1;
    }

    universes_with_k_turns_to_21
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 739785);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 444356092776315);
    }

    fn data() -> String {
"Player 1 starting position: 4
Player 2 starting position: 8".to_string()
    }
}

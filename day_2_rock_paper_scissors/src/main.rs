fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut total_score = 0;

    loop {
        let mut round = String::new();

        std::io::stdin()
            .read_line(&mut round)
            .expect("Failed to read line");

        let round = round.trim();
        if round.is_empty() {
            break;
        }

        let mut hand_shapes_splitter = round.split(' ');
        let opponent_hand_shape = match hand_shapes_splitter.next().unwrap() {
            "A" => HandShape::Rock,
            "B" => HandShape::Paper,
            _ => HandShape::Scissors,
        };

        let target_round_outcome = match hand_shapes_splitter.next().unwrap() {
            "X" => RoundOutcome::Loss,
            "Y" => RoundOutcome::Draw,
            _ => RoundOutcome::Win,
        };

        let my_hand_shape = determine_my_hand_shape(opponent_hand_shape, target_round_outcome);
        let hand_shape_score = match my_hand_shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };

        let round_outcome_score = match target_round_outcome {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            RoundOutcome::Loss => 0,
        };

        let round_score = hand_shape_score + round_outcome_score;
        total_score += round_score;
    }

    println!("{total_score}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut total_score = 0;

    loop {
        let mut round = String::new();

        std::io::stdin()
            .read_line(&mut round)
            .expect("Failed to read line");

        let round = round.trim();
        if round.is_empty() {
            break;
        }

        let mut hand_shapes_splitter = round.split(' ');
        let opponent_hand_shape = match hand_shapes_splitter.next().unwrap() {
            "A" => HandShape::Rock,
            "B" => HandShape::Paper,
            _ => HandShape::Scissors,
        };

        let my_hand_shape = match hand_shapes_splitter.next().unwrap() {
            "X" => HandShape::Rock,
            "Y" => HandShape::Paper,
            _ => HandShape::Scissors,
        };

        let round_outcome = determine_round_outcome(opponent_hand_shape, my_hand_shape);
        let round_outcome_score = match round_outcome {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            RoundOutcome::Loss => 0,
        };

        let hand_shape_score = match my_hand_shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };

        let round_score = hand_shape_score + round_outcome_score;
        total_score += round_score;
    }

    println!("{total_score}");
}

fn determine_my_hand_shape(
    opponent_hand_shape: HandShape,
    target_round_outcome: RoundOutcome,
) -> HandShape {
    if target_round_outcome == RoundOutcome::Win {
        return match opponent_hand_shape {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        };
    }

    if target_round_outcome == RoundOutcome::Loss {
        return match opponent_hand_shape {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
        };
    }

    opponent_hand_shape
}

fn determine_round_outcome(
    opponent_hand_shape: HandShape,
    my_hand_shape: HandShape,
) -> RoundOutcome {
    if (my_hand_shape == HandShape::Rock && opponent_hand_shape == HandShape::Scissors)
        || (my_hand_shape == HandShape::Scissors && opponent_hand_shape == HandShape::Paper)
        || (my_hand_shape == HandShape::Paper && opponent_hand_shape == HandShape::Rock)
    {
        return RoundOutcome::Win;
    }

    if my_hand_shape != opponent_hand_shape {
        return RoundOutcome::Loss;
    }

    RoundOutcome::Draw
}

#[derive(Clone, Copy, PartialEq)]
enum RoundOutcome {
    Win,
    Draw,
    Loss,
}

#[derive(Clone, Copy, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

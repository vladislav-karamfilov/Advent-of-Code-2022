fn main() {
    solve_puzzle1();
}

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

        let hand_shape_score: i32 = match my_hand_shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            _ => 3,
        };

        let round_outcome_score: i32 = match round_outcome {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            _ => 0,
        };

        total_score += hand_shape_score;
        total_score += round_outcome_score;
    }

    println!("{total_score}");
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

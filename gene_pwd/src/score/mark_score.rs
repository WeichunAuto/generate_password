use zxcvbn::{Score, zxcvbn};

pub fn evoluate_score(pwd: &str) -> Score {
    let estimate = zxcvbn(pwd, &[]);
    estimate.score()
}

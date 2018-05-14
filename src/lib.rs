use std::cmp::Ordering;

pub fn apportion(votes: &Vec<u64>, seat_number: u64) -> Vec<u64> {
    let total = votes.iter().fold(0, |acc, &x| acc + x);
    let hare_quota = total as f64 / seat_number as f64;

    let votes_quota: Vec<f64> = votes
        .iter()
        .map(|vote| (*vote as f64) / hare_quota)
        .collect();

    // calculate automatic seats first
    let mut seats: Vec<u64> = votes_quota.iter().map(|v| v.floor() as u64).collect();

    let mut remainders: Vec<(f64, u64)> = votes_quota
        .iter()
        .enumerate()
        .map(|(i, v)| (v.fract(), i as u64))
        .collect();

    let remaining_seat: u64 = seat_number - seats.iter().fold(0, |acc, &x| acc + x);

    remainders.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    remainders.reverse();

    let highest_remainder_seats: Vec<u64> = remainders
        .iter()
        .take(remaining_seat as usize)
        .map(|v| v.1)
        .collect();

    for index in highest_remainder_seats {
        seats[index as usize] += 1;
    }

    seats
}

#[cfg(test)]
mod tests {
    use super::apportion;

    #[test]
    fn test_apportion() {
        let votes = vec![47_000, 16_000, 15_800, 12_000, 6_100, 3_100];
        let seats = 10;
        let total_seats = apportion(&votes, seats);
        assert_eq!(vec![5, 2, 1, 1, 1, 0], total_seats);
    }

    #[test]
    fn test_apportion_again() {
        let votes = vec![1500, 1500, 900, 500, 500, 200];
        let seats = 25;
        let total_seats = apportion(&votes, seats);
        assert_eq!(vec![7, 7, 4, 3, 3, 1], total_seats);

        let new_seats = 26;
        let new_total_seats = apportion(&votes, new_seats);
        assert_eq!(vec![8, 8, 5, 2, 2, 1], new_total_seats);
    }
}

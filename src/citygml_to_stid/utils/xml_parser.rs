use kasane_logic::point::Coordinate;

pub fn parse_points(input: &str) -> Result<Vec<Coordinate>, Box<dyn std::error::Error>> {
    let nums: Vec<f64> = input
        .split_whitespace()
        .map(str::parse::<f64>)
        .collect::<Result<_, _>>()?;
    if !nums.len().is_multiple_of(3) {
        return Err(format!("入力数が3の倍数ではありません: {}", nums.len()).into());
    }
    Ok(nums
        .chunks(3)
        .map(|c| Coordinate {
            latitude: c[0],
            longitude: c[1],
            altitude: c[2],
        })
        .collect())
}

# Rust

## Run
```cargo run "<Path to puzzle input><day number>.txt"```

## Test
```cargo test```

### day template
```
pub struct DayResult {
    pub total: u64,
    pub part_two_total: u64,
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        part_two_total: 0,
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should() {}
}
```

# C#

## Run
```dotnet run "/mnt/Video games/git/AoC2025/PUZZLE_INPUT" --project runner```

## Test
```dotnet test```




pub fn get_nth(n: usize) -> u64 {
    
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut state = (0, 1); // Represents (current, next)
    
    for _ in 2..=n {
        state = (state.1, state.0 + state.1);
    }

    state.1
}


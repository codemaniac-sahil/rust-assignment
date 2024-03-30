fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}

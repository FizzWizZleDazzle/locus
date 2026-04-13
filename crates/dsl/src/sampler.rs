//! Random variable sampling — all sampler types from DSL spec section 4.1

use crate::error::DslError;
use crate::spec::Difficulty;

/// Sample a difficulty value from a Difficulty spec
pub fn sample_difficulty(diff: &Difficulty) -> Result<i32, DslError> {
    match diff {
        Difficulty::Value(v) => Ok(*v),
        Difficulty::Label(label) => {
            let (lo, hi) = match label.as_str() {
                "very_easy" => (800, 1000),
                "easy" => (1000, 1200),
                "medium" => (1200, 1400),
                "hard" => (1400, 1600),
                "very_hard" => (1600, 1800),
                "competition" => (1800, 2200),
                other => return Err(DslError::InvalidDifficulty(other.into())),
            };
            Ok(rand::random_range(lo..=hi))
        }
        Difficulty::Range(s) => {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 2 {
                let lo: i32 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| DslError::InvalidDifficulty(s.clone()))?;
                let hi: i32 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| DslError::InvalidDifficulty(s.clone()))?;
                Ok(rand::random_range(lo..=hi))
            } else {
                Err(DslError::InvalidDifficulty(s.clone()))
            }
        }
    }
}

/// Parse a sampler definition string and produce a sampled value.
///
/// Recognized formats:
///   integer(lo, hi)        → random integer in [lo, hi]
///   nonzero(lo, hi)        → random integer in [lo, hi], excludes 0
///   decimal(lo, hi, places) → random decimal
///   choice(a, b, c, ...)   → pick from list
///   prime(lo, hi)          → random prime in range
///   rational(lo, hi, max_d) → random simplified fraction
///   vector(dim, lo, hi)    → random integer vector
///   matrix(rows, cols, lo, hi) → random integer matrix
///
/// Returns the sampled value as a string (compatible with SymEngine parsing).
pub fn sample(definition: &str) -> Result<String, DslError> {
    let def = definition.trim();

    // Check for function-style sampler: name(args)
    if let Some(paren_pos) = def.find('(') {
        if !def.ends_with(')') {
            return Err(DslError::InvalidSampler(def.to_string()));
        }
        let name = &def[..paren_pos];
        let args_str = &def[paren_pos + 1..def.len() - 1];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        match name {
            "integer" => sample_integer(&args),
            "nonzero" => sample_nonzero(&args),
            "decimal" => sample_decimal(&args),
            "choice" => sample_choice(&args),
            "prime" => sample_prime(&args),
            "rational" => sample_rational(&args),
            "vector" => sample_vector(&args),
            "matrix" => sample_matrix(&args),
            "angle" => sample_angle(&args),
            _ => Err(DslError::InvalidSampler(format!("Unknown sampler: {name}"))),
        }
    } else {
        // Not a sampler — this is a derived expression, return as-is
        Err(DslError::InvalidSampler(format!(
            "Not a sampler expression: {def}"
        )))
    }
}

/// Check if a definition string is a sampler (vs a derived expression)
pub fn is_sampler(definition: &str) -> bool {
    let def = definition.trim();
    if let Some(paren) = def.find('(') {
        let name = &def[..paren];
        matches!(
            name,
            "integer"
                | "nonzero"
                | "decimal"
                | "choice"
                | "prime"
                | "rational"
                | "vector"
                | "matrix"
                | "angle"
        )
    } else {
        false
    }
}

fn sample_integer(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::InvalidSampler(
            "integer(lo, hi) requires 2 args".into(),
        ));
    }
    let lo: i64 = args[0].parse().map_err(|_| {
        DslError::InvalidSampler(format!("integer: can't parse lo '{}'", args[0]))
    })?;
    let hi: i64 = args[1].parse().map_err(|_| {
        DslError::InvalidSampler(format!("integer: can't parse hi '{}'", args[1]))
    })?;
    Ok(rand::random_range(lo..=hi).to_string())
}

fn sample_nonzero(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::InvalidSampler(
            "nonzero(lo, hi) requires 2 args".into(),
        ));
    }
    let lo: i64 = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("nonzero: can't parse '{}'", args[0])))?;
    let hi: i64 = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("nonzero: can't parse '{}'", args[1])))?;

    for _ in 0..1000 {
        let v = rand::random_range(lo..=hi);
        if v != 0 {
            return Ok(v.to_string());
        }
    }
    Err(DslError::ConstraintUnsatisfiable {
        constraint: "nonzero".into(),
        attempts: 1000,
    })
}

fn sample_decimal(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 3 {
        return Err(DslError::InvalidSampler(
            "decimal(lo, hi, places) requires 3 args".into(),
        ));
    }
    let lo: f64 = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("decimal: can't parse '{}'", args[0])))?;
    let hi: f64 = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("decimal: can't parse '{}'", args[1])))?;
    let places: u32 = args[2]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("decimal: can't parse '{}'", args[2])))?;

    let scale = 10f64.powi(places as i32);
    let lo_int = (lo * scale) as i64;
    let hi_int = (hi * scale) as i64;
    let v = rand::random_range(lo_int..=hi_int) as f64 / scale;
    Ok(format!("{:.prec$}", v, prec = places as usize))
}

fn sample_choice(args: &[&str]) -> Result<String, DslError> {
    if args.is_empty() {
        return Err(DslError::InvalidSampler("choice() requires args".into()));
    }
    let idx = rand::random_range(0..args.len());
    Ok(args[idx].to_string())
}

fn sample_prime(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::InvalidSampler(
            "prime(lo, hi) requires 2 args".into(),
        ));
    }
    let lo: u64 = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("prime: can't parse '{}'", args[0])))?;
    let hi: u64 = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("prime: can't parse '{}'", args[1])))?;

    let primes: Vec<u64> = (lo..=hi).filter(|&n| is_prime(n)).collect();
    if primes.is_empty() {
        return Err(DslError::InvalidSampler(format!(
            "No primes in range [{lo}, {hi}]"
        )));
    }
    let idx = rand::random_range(0..primes.len());
    Ok(primes[idx].to_string())
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn sample_rational(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 3 {
        return Err(DslError::InvalidSampler(
            "rational(lo, hi, max_denom) requires 3 args".into(),
        ));
    }
    let lo: i64 = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("rational: can't parse '{}'", args[0])))?;
    let hi: i64 = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("rational: can't parse '{}'", args[1])))?;
    let max_d: i64 = args[2]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("rational: can't parse '{}'", args[2])))?;

    let denom = rand::random_range(2..=max_d);
    let num_lo = lo * denom;
    let num_hi = hi * denom;
    let num = rand::random_range(num_lo..=num_hi);

    let g = gcd(num.unsigned_abs(), denom.unsigned_abs()) as i64;
    Ok(format!("{}/{}", num / g, denom / g))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn sample_vector(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 3 {
        return Err(DslError::InvalidSampler(
            "vector(dim, lo, hi) requires 3 args".into(),
        ));
    }
    let dim: usize = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("vector: can't parse '{}'", args[0])))?;
    let lo: i64 = args[1].parse().map_err(|_| {
        DslError::InvalidSampler(format!("vector: can't parse '{}'", args[1]))
    })?;
    let hi: i64 = args[2].parse().map_err(|_| {
        DslError::InvalidSampler(format!("vector: can't parse '{}'", args[2]))
    })?;

    let vals: Vec<String> = (0..dim)
        .map(|_| rand::random_range(lo..=hi).to_string())
        .collect();
    Ok(format!("[{}]", vals.join(", ")))
}

fn sample_matrix(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 4 {
        return Err(DslError::InvalidSampler(
            "matrix(rows, cols, lo, hi) requires 4 args".into(),
        ));
    }
    let rows: usize = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("matrix: can't parse '{}'", args[0])))?;
    let cols: usize = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("matrix: can't parse '{}'", args[1])))?;
    let lo: i64 = args[2].parse().map_err(|_| {
        DslError::InvalidSampler(format!("matrix: can't parse '{}'", args[2]))
    })?;
    let hi: i64 = args[3].parse().map_err(|_| {
        DslError::InvalidSampler(format!("matrix: can't parse '{}'", args[3]))
    })?;

    let row_strs: Vec<String> = (0..rows)
        .map(|_| {
            let vals: Vec<String> = (0..cols)
                .map(|_| rand::random_range(lo..=hi).to_string())
                .collect();
            format!("[{}]", vals.join(", "))
        })
        .collect();
    Ok(format!("[{}]", row_strs.join(", ")))
}

fn sample_angle(args: &[&str]) -> Result<String, DslError> {
    if args.len() != 3 {
        return Err(DslError::InvalidSampler(
            "angle(lo, hi, step) requires 3 args".into(),
        ));
    }
    let lo: i64 = args[0]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("angle: can't parse '{}'", args[0])))?;
    let hi: i64 = args[1]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("angle: can't parse '{}'", args[1])))?;
    let step: i64 = args[2]
        .parse()
        .map_err(|_| DslError::InvalidSampler(format!("angle: can't parse '{}'", args[2])))?;

    let steps = (hi - lo) / step;
    let n = rand::random_range(0..=steps);
    Ok((lo + n * step).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_integer() {
        let v = sample("integer(1, 10)").unwrap();
        let n: i64 = v.parse().unwrap();
        assert!((1..=10).contains(&n));
    }

    #[test]
    fn test_sample_nonzero() {
        for _ in 0..100 {
            let v = sample("nonzero(-3, 3)").unwrap();
            let n: i64 = v.parse().unwrap();
            assert_ne!(n, 0);
            assert!((-3..=3).contains(&n));
        }
    }

    #[test]
    fn test_sample_choice() {
        let v = sample("choice(2, 3, 5, 7)").unwrap();
        assert!(["2", "3", "5", "7"].contains(&v.as_str()));
    }

    #[test]
    fn test_sample_prime() {
        let v = sample("prime(2, 20)").unwrap();
        let n: u64 = v.parse().unwrap();
        assert!(is_prime(n));
        assert!((2..=20).contains(&n));
    }

    #[test]
    fn test_sample_vector() {
        let v = sample("vector(3, -5, 5)").unwrap();
        assert!(v.starts_with('[') && v.ends_with(']'));
    }

    #[test]
    fn test_sample_matrix() {
        let v = sample("matrix(2, 2, -5, 5)").unwrap();
        assert!(v.starts_with("[["));
    }

    #[test]
    fn test_is_sampler() {
        assert!(is_sampler("integer(1, 10)"));
        assert!(is_sampler("choice(a, b, c)"));
        assert!(!is_sampler("a + b"));
        assert!(!is_sampler("derivative(f, x)"));
    }

    #[test]
    fn test_difficulty_label() {
        let d = sample_difficulty(&Difficulty::Label("medium".into())).unwrap();
        assert!((1200..=1400).contains(&d));
    }
}

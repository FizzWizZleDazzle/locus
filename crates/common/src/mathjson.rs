use serde_json::Value;

/// Convert MathJSON to SymEngine-compatible plain text notation
pub fn convert_mathjson_to_plain(json_str: &str) -> Result<String, String> {
    let json: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    node_to_symengine(&json)
}

/// Convert a MathJSON node to SymEngine notation
fn node_to_symengine(node: &Value) -> Result<String, String> {
    // Number: {"num": "42"} → "42"
    if let Some(num) = node.get("num") {
        return Ok(num.as_str().unwrap_or("0").to_string());
    }

    // Symbol: {"sym": "x"} → "x"
    if let Some(sym) = node.get("sym") {
        return Ok(sym.as_str().unwrap_or("").to_string());
    }

    // String (for special constants): {"str": "Pi"} → "pi"
    if let Some(s) = node.get("str") {
        let str_val = s.as_str().unwrap_or("");
        return match str_val {
            "Pi" => Ok("pi".to_string()),
            "E" | "ExponentialE" => Ok("E".to_string()),
            _ => Ok(str_val.to_string()),
        };
    }

    // Function: {"fn": "Add", "args": [...]}
    if let Some(fn_name) = node.get("fn") {
        let fn_str = fn_name.as_str().unwrap_or("");
        let args = node
            .get("args")
            .and_then(|a| a.as_array())
            .ok_or_else(|| format!("Missing args for function: {}", fn_str))?;
        return convert_function(fn_str, args);
    }

    Err("Unknown MathJSON node type".to_string())
}

/// Convert a MathJSON function to SymEngine notation
fn convert_function(name: &str, args: &[Value]) -> Result<String, String> {
    match name {
        "Add" => binary_op_chain(args, "+"),
        "Subtract" => {
            if args.len() == 1 {
                // Unary minus: {"fn": "Subtract", "args": [x]} → "-x"
                let arg = node_to_symengine(&args[0])?;
                Ok(format!("-({})", arg))
            } else {
                binary_op(args, "-")
            }
        }
        "Negate" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("-({})", arg))
        }
        "Multiply" => binary_op_chain(args, "*"),
        "Divide" => {
            let a = node_to_symengine(&args[0])?;
            let b = node_to_symengine(&args[1])?;
            Ok(format!("({})/({})", a, b))
        }
        "Power" => {
            let base = node_to_symengine(&args[0])?;
            let exp = node_to_symengine(&args[1])?;
            // Check if exponent is 1/2 for sqrt
            if exp == "0.5" || exp == "(1)/(2)" {
                Ok(format!("sqrt({})", base))
            } else {
                Ok(format!("({})^({})", base, exp))
            }
        }
        "Sqrt" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("sqrt({})", arg))
        }
        "Root" => {
            // Root(x, n) = x^(1/n)
            let base = node_to_symengine(&args[0])?;
            let n = node_to_symengine(&args[1])?;
            Ok(format!("({})^(1/({}))", base, n))
        }
        "Exp" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("exp({})", arg))
        }
        "Ln" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("log({})", arg))
        }
        "Log" => {
            if args.len() == 1 {
                // Log base 10
                let arg = node_to_symengine(&args[0])?;
                Ok(format!("log({}, 10)", arg))
            } else {
                // Log with custom base
                let arg = node_to_symengine(&args[0])?;
                let base = node_to_symengine(&args[1])?;
                Ok(format!("log({}, {})", arg, base))
            }
        }
        "Sin" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("sin({})", arg))
        }
        "Cos" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("cos({})", arg))
        }
        "Tan" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("tan({})", arg))
        }
        "Arcsin" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("asin({})", arg))
        }
        "Arccos" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("acos({})", arg))
        }
        "Arctan" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("atan({})", arg))
        }
        "Sec" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("sec({})", arg))
        }
        "Csc" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("csc({})", arg))
        }
        "Cot" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("cot({})", arg))
        }
        "Abs" => {
            let arg = node_to_symengine(&args[0])?;
            Ok(format!("abs({})", arg))
        }
        _ => Err(format!("Unsupported function: {}", name)),
    }
}

/// Handle binary operations (2 arguments)
fn binary_op(args: &[Value], op: &str) -> Result<String, String> {
    if args.len() != 2 {
        return Err(format!("Expected 2 args for {}, got {}", op, args.len()));
    }
    let a = node_to_symengine(&args[0])?;
    let b = node_to_symengine(&args[1])?;
    Ok(format!("({}{}{})", a, op, b))
}

/// Handle operations with multiple arguments (chained)
fn binary_op_chain(args: &[Value], op: &str) -> Result<String, String> {
    if args.is_empty() {
        return Err(format!("No args for {}", op));
    }
    if args.len() == 1 {
        return node_to_symengine(&args[0]);
    }

    let mut result = String::from("(");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            result.push_str(op);
        }
        let converted = node_to_symengine(arg)?;
        result.push_str(&converted);
    }
    result.push(')');
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let json = r#"{"num": "42"}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "42");
    }

    #[test]
    fn test_symbol() {
        let json = r#"{"sym": "x"}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "x");
    }

    #[test]
    fn test_add() {
        let json = r#"{"fn": "Add", "args": [{"sym": "x"}, {"num": "2"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x+2)");
    }

    #[test]
    fn test_subtract() {
        let json = r#"{"fn": "Subtract", "args": [{"sym": "x"}, {"num": "2"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x-2)");
    }

    #[test]
    fn test_negate() {
        let json = r#"{"fn": "Negate", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "-(x)");
    }

    #[test]
    fn test_multiply() {
        let json = r#"{"fn": "Multiply", "args": [{"sym": "x"}, {"num": "2"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x*2)");
    }

    #[test]
    fn test_divide() {
        let json = r#"{"fn": "Divide", "args": [{"sym": "x"}, {"num": "2"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x)/(2)");
    }

    #[test]
    fn test_power() {
        let json = r#"{"fn": "Power", "args": [{"sym": "x"}, {"num": "2"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x)^(2)");
    }

    #[test]
    fn test_sqrt() {
        let json = r#"{"fn": "Sqrt", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "sqrt(x)");
    }

    #[test]
    fn test_nested_multiply_add() {
        // (x+2)(x+3)
        let json = r#"{"fn": "Multiply", "args": [
            {"fn": "Add", "args": [{"sym": "x"}, {"num": "2"}]},
            {"fn": "Add", "args": [{"sym": "x"}, {"num": "3"}]}
        ]}"#;
        assert_eq!(
            convert_mathjson_to_plain(json).unwrap(),
            "((x+2)*(x+3))"
        );
    }

    #[test]
    fn test_fraction() {
        // x^2/2
        let json = r#"{"fn": "Divide", "args": [
            {"fn": "Power", "args": [{"sym": "x"}, {"num": "2"}]},
            {"num": "2"}
        ]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "((x)^(2))/(2)");
    }

    #[test]
    fn test_sin() {
        let json = r#"{"fn": "Sin", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "sin(x)");
    }

    #[test]
    fn test_cos() {
        let json = r#"{"fn": "Cos", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "cos(x)");
    }

    #[test]
    fn test_tan() {
        let json = r#"{"fn": "Tan", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "tan(x)");
    }

    #[test]
    fn test_ln() {
        let json = r#"{"fn": "Ln", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "log(x)");
    }

    #[test]
    fn test_exp() {
        let json = r#"{"fn": "Exp", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "exp(x)");
    }

    #[test]
    fn test_abs() {
        let json = r#"{"fn": "Abs", "args": [{"sym": "x"}]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "abs(x)");
    }

    #[test]
    fn test_chain_add() {
        // x + y + z
        let json = r#"{"fn": "Add", "args": [
            {"sym": "x"},
            {"sym": "y"},
            {"sym": "z"}
        ]}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "(x+y+z)");
    }

    #[test]
    fn test_pi() {
        let json = r#"{"str": "Pi"}"#;
        assert_eq!(convert_mathjson_to_plain(json).unwrap(), "pi");
    }

    #[test]
    fn test_invalid_json() {
        let json = r#"{"invalid"}"#;
        assert!(convert_mathjson_to_plain(json).is_err());
    }

    #[test]
    fn test_unsupported_function() {
        let json = r#"{"fn": "UnsupportedFunc", "args": [{"sym": "x"}]}"#;
        assert!(convert_mathjson_to_plain(json).is_err());
    }
}

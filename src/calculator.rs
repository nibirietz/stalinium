fn calculate_polish_notation(expression: &String) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    let mut is_negative = false;
    let mut number = 0f32;

    for i in 0..=(expression.len() - 1) {
        if expression.as_bytes()[i].is_ascii_digit() {
            number *= 10f32;
            number += (expression.as_bytes()[i] as u32 - 0x30) as f32;
        } else if expression.as_bytes()[i] as char == '-'
            && i != expression.len() - 1
            && expression.as_bytes()[i + 1].is_ascii_digit()
        {
            is_negative = true;
        } else if expression.as_bytes()[i] as char == ' ' {
            if expression.as_bytes()[i - 1].is_ascii_digit() {
                if is_negative {
                    number = -number;
                }
                is_negative = false;
                stack.push(number);
                number = 0f32;
            }
        } else if expression.as_bytes()[i] as char == '+'
            || expression.as_bytes()[i] as char == '-'
            || expression.as_bytes()[i] as char == '*'
            || expression.as_bytes()[i] as char == '/'
        {
            let num1 = stack.pop().unwrap();
            let num2 = stack.pop().unwrap();
            let result = calculate_operations(num2, num1, expression.as_bytes()[i] as char);
            match result {
                None => panic!("m"),
                _ => stack.push(result.unwrap()),
            }
        }
    }

    return stack[0];
}

fn calculate_operations(num1: f32, num2: f32, operator: char) -> Option<f32> {
    if operator == '+' {
        Some(num1 + num2)
    } else if operator == '-' {
        Some(num1 - num2)
    } else if operator == '*' {
        Some(num1 * num2)
    } else {
        match num2 {
            0.0 => None,
            _ => Some(num1 / num2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpn() {
        assert_eq!(calculate_polish_notation(&String::from("5 4 +")), 9.0);
        assert_eq!(calculate_polish_notation(&String::from("5 3 -")), 2.0);
    }
}

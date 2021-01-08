pub fn evaluate(x: &[u8]) -> usize {
    let (remaining, result) = evaluate_expr(x);
    assert!(remaining.is_empty());
    result
}

fn evaluate_expr(xs: &[u8]) -> (&[u8], usize) {
    match xs {
        [b' ', xs @ ..] => evaluate_expr(xs),
        [a @ b'0'..=b'9', xs @ ..] => evaluate_partial(xs, (*a - b'0') as usize),
        [b'(', xs @ ..] => {
            let (xs, a) = evaluate_group(xs);
            evaluate_partial(xs, a)
        }
        _ => panic!("Invalid expression"),
    }
}

fn evaluate_group(xs: &[u8]) -> (&[u8], usize) {
    match evaluate_expr(xs) {
        ([b')', xs @ ..], a) => (xs, a),
        _ => panic!("Invalid group"),
    }
}

fn evaluate_partial(xs: &[u8], a: usize) -> (&[u8], usize) {
    match xs {
        [] => (xs, a),
        [b')', ..] => (xs, a),
        [b' ', xs @ ..] => evaluate_partial(xs, a),
        [b'+', xs @ ..] => evaluate_partial_sum(xs, a),
        [b'*', xs @ ..] => evaluate_partial_product(xs, a),
        _ => panic!("Invalid partial"),
    }
}

fn evaluate_partial_sum(xs: &[u8], a: usize) -> (&[u8], usize) {
    match xs {
        [b' ', xs @ ..] => evaluate_partial_sum(xs, a),
        [b @ b'0'..=b'9', xs @ ..] => evaluate_partial(xs, a + (*b - b'0') as usize),
        [b'(', xs @ ..] => {
            let (xs, b) = evaluate_group(xs);
            evaluate_partial(xs, a + b)
        }
        _ => panic!("Invalid partial sum"),
    }
}

fn evaluate_partial_product(xs: &[u8], a: usize) -> (&[u8], usize) {
    match xs {
        [b' ', xs @ ..] => evaluate_partial_product(xs, a),
        [b @ b'0'..=b'9', xs @ ..] => evaluate_partial(xs, a * (*b - b'0') as usize),
        [b'(', xs @ ..] => {
            let (xs, b) = evaluate_group(xs);
            evaluate_partial(xs, a * b)
        }
        _ => panic!("Invalid partial product"),
    }
}

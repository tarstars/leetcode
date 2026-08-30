use crate::Solution;

fn non_zero(a: &Vec<u32>) -> bool {
    a.len() != 0 && (a.len() > 1 || a[0] != 0)
}

fn odd(a: &Vec<u32>) -> bool {
    a.last().unwrap() % 2 != 0
}

fn increment(mut a: Vec<u32>, mut b: Vec<u32>) -> Vec<u32> {
    let mut c: Vec<u32> = Vec::new();
    let mut vc: u32 = 0;

    while !a.is_empty() && !b.is_empty() {
        vc += a.pop().unwrap() + b.pop().unwrap();
        c.push(vc % 10);
        vc /= 10;
    }

    while !a.is_empty() {
        vc += a.pop().unwrap();
        c.push(vc % 10);
        vc /= 10;
    }

    while !b.is_empty() {
        vc += b.pop().unwrap();
        c.push(vc % 10);
        vc /= 10;
    }

    if vc != 0 {
        c.push(vc);
    }

    c.reverse();
    c
}

fn div2(a: Vec<u32>) -> Vec<u32> {
    let mut cv = 0;
    let mut b: Vec<u32> = Vec::new();
    let mut lead_z = true;

    for v in a {
        let nv = (v + cv*10) / 2;
        cv = (v + cv*10) % 2;

        if nv == 0 && lead_z {
            continue;
        }

        lead_z = false;
        b.push(nv);
    }

    if b.len() != 0 {b} else {vec![0]}
}

fn mul2(mut a: Vec<u32>) -> Vec<u32> {
    let mut b: Vec<u32> = Vec::new();

    while let Some(v) = a.pop() {
        b.push(v * 2)
    }

    let mut c: Vec<u32> = Vec::new();
    let mut cv = 0;
    for v in b {
        c.push((v + cv) % 10);
        cv = (v + cv) / 10;
    }
    if cv != 0 {
        c.push(cv);
    }

    c.reverse();
    c
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut a: Vec<u32> = num1
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut b: Vec<u32> = num2
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut c: Vec<u32> = vec![0];

        while non_zero(&a) {
            if odd(&a) {
                c = increment(c, b.clone());
            }
            a = div2(a);
            b = mul2(b);
        }

        c.iter()
            .map(|&d| char::from_digit(d, 10).unwrap())
            .collect()
    }
}

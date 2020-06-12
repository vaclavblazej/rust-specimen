
fn is_prime(value: i64) -> bool {
    let mut i=2;
    while i*i <= value {
        if value % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn chain2(value: i64) -> Vec<i64> {
    let mut val = value;
    if val == 1 {
        return vec!(1);
    }
    let mut res;
    if val % 2 == 0 {
        res = chain2(value/2);
    } else {
        res = chain2(value*3+1);
    }
    res.push(value);
    return res;
}

fn chain(value: i64) -> Vec<i64> {
    let mut val = value;
    let mut res = vec!();
    loop {
        res.push(val);
        if val == 1 {
            break;
        }
        if val % 2 == 0 {
            val = val / 2;
        } else {
            val = val * 3 + 1;
        }
    }
    return res;
}

fn main() {
    let to = 10;
    for i in 1..=to {
        println!("{}: {:?}", i, chain2(i));
    }
    // for i in 1..=to {
    //     println!("{}: {:?}", i, is_prime(i));
    // }
}

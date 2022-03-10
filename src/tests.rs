use crate::Uint256;
use rand::Rng;
use std::process::Command;
use std::str;

fn rand_uint256() -> Uint256 {
    let mut rng = rand::thread_rng();
    Uint256([rng.gen::<u128>(), rng.gen::<u128>()])
}

fn python(script: &str) -> String {
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .output()
        .unwrap();
    str::from_utf8(&output.stdout).unwrap().trim().to_string()
}

#[test]
fn test_from_hex() {
    let num = "0x4412a71f5b52042b96bccc513e81344b124e94a808b6f8be44a1fcf131629797";
    let a = Uint256::from_hex(num);
    assert_eq!(a.hex(), num);
}

#[test]
fn test_bit_len() {
    for _ in 1..100 {
        let n = rand_uint256();
        let a = n.bit_len();
        let b = python(&format!("print({}.bit_length())", n));
        assert_eq!(a.to_string(), b)
    }
}

#[test]
fn test_shr() {
    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rng.gen_range(0..256);
        let ans = a >> b;
        let correct = python(&format!("print(hex({} >> {}))", a, b));
        assert_eq!(correct, ans.hex());
    }
}

#[test]
fn test_shl() {
    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rng.gen_range(0..256);
        let ans = a << b;
        let correct = python(&format!("print(hex(({} << {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, ans.hex());
    }
}

#[test]
fn test_add() {
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rand_uint256();
        let c = a + b;
        let correct = python(&format!("print(hex(({} + {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, c.hex());
    }
}

#[test]
fn test_add_assign() {
    for _ in 1..100 {
        let mut a = rand_uint256();
        let b = rand_uint256();
        let correct = python(&format!("print(hex(({} + {}) & ((1 << 256) - 1)))", a, b));
        a += b;
        assert_eq!(correct, a.hex());
    }
}

#[test]
fn test_sub() {
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rand_uint256();
        let c = a - b;
        let correct = python(&format!("print(hex(({} - {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, c.hex());
    }
}

#[test]
fn test_sub_assign() {
    for _ in 1..100 {
        let mut a = rand_uint256();
        let b = rand_uint256();
        let correct = python(&format!("print(hex(({} - {}) & ((1 << 256) - 1)))", a, b));
        a -= b;
        assert_eq!(correct, a.hex());
    }
}

#[test]
fn test_and() {
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rand_uint256();
        let c = a & b;
        let correct = python(&format!("print(hex(({} & {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, c.hex());
    }
}

#[test]
fn test_and_assign() {
    for _ in 1..100 {
        let mut a = rand_uint256();
        let b = rand_uint256();
        let correct = python(&format!("print(hex(({} & {}) & ((1 << 256) - 1)))", a, b));
        a &= b;
        assert_eq!(correct, a.hex());
    }
}

#[test]
fn test_or() {
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rand_uint256();
        let c = a | b;
        let correct = python(&format!("print(hex(({} | {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, c.hex());
    }
}

#[test]
fn test_or_assign() {
    for _ in 1..100 {
        let mut a = rand_uint256();
        let b = rand_uint256();
        let correct = python(&format!("print(hex(({} | {}) & ((1 << 256) - 1)))", a, b));
        a |= b;
        assert_eq!(correct, a.hex());
    }
}

#[test]
fn test_xor() {
    for _ in 1..100 {
        let a = rand_uint256();
        let b = rand_uint256();
        let c = a ^ b;
        let correct = python(&format!("print(hex(({} ^ {}) & ((1 << 256) - 1)))", a, b));
        assert_eq!(correct, c.hex());
    }
}

#[test]
fn test_xor_assign() {
    for _ in 1..100 {
        let mut a = rand_uint256();
        let b = rand_uint256();
        let correct = python(&format!("print(hex(({} ^ {}) & ((1 << 256) - 1)))", a, b));
        a ^= b;
        assert_eq!(correct, a.hex());
    }
}

#[test]
fn test_not() {
    for _ in 1..100 {
        let a = rand_uint256();
        let c = !a;
        let correct = a ^ Uint256::from_hex(&format!("0x{}", "f".repeat(64)));
        assert_eq!(correct.hex(), c.hex());
    }
}

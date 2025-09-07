#![allow(unused)]

// Question operator - ?
fn f1() -> Result<i32, String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<i32, String> {
    println!("f2");
    Ok(2)
}

fn f1_f2_match() -> Result<i32, String> {
    let res_1 = f1();
    let out_1 = match res_1 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f1".to_string());
        }
    };
    let res_2 = f2();
    let out_2 = match res_2 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f2".to_string());
        }
    };
    Ok(out_1 + out_2)
}
fn f1_f2_question() -> Result<i32, String> {
    let out_1 = f1()?;
    let out_2 = f2()?;
    Ok(out_1 + out_2)
}

fn main() {
    let res = f1_f2_question();
    println!("res: {:?}", res);
}

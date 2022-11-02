#![cfg_attr(not(test), no_main)]

pub fn int_to_chinese_string(number: i64) -> String {
    int_number_convert(number)
}
pub fn float_to_chinese_string(number: f64) -> String {
    let mut number = number;
    let mut sum = String::new();
    if number as i64 != 0 {
        sum = int_number_convert(number as i64);
    }
    number = number * 100.00;
    let sum2 = float_number_convert(number as i64);
    sum + &sum2
}

fn int_number_convert(number: i64) -> String {
    let mut number = number;
    if number == 0 {
        return String::from("零元");
    }
    let mut sum = String::new();
    let mut acc = 1;
    let mut one = number % 10;
    loop {
        //判断是否保留零比如100001那么就是十万零壹而不是有很多个零
        if (acc % 4 != 1 || number % 10 != 0) && (number % 10 != 0 || one != 0) {
            sum += &match_number(number % 10);
        }
        //判断是否到达最后一位数，如果是则退出
        if number < 10 {
            break;
        }
        //这个主要是判断单位是否保留，比如壹亿零壹元，万这个单位是不需要的，我需要判断万这个范围内有没有数字不为零，如果都为零的话万这个单位需要去掉
        //目前还在想怎么优化，但是只要有一个为true就直接执行了，对性能没有什么损失
        if acc % 4 == 0
            && (number / 10 % 10 != 0
                || number / 100 % 10 != 0
                || number / 1000 % 10 != 0
                || number / 10000 % 10 != 0)
        {
            sum += &match_unit(acc);
        } else if number / 10 % 10 != 0 {
            //这里是判断关于十，百，千。比如壹十或者壹千
            sum += &match_unit_number(acc);
        }
        one = number % 10;
        acc += 1;
        number /= 10;
    }
    let sum: String = sum.chars().rev().collect();
    sum + "元"
}
//用于浮点数小数的计算
fn float_number_convert(number: i64) -> String {
    let mut number = number;
    let mut sum = String::new();
    let mut acc = 1;
    while acc <= 2 {
        if number % 10 != 0 {
            sum += &match_unit_float(acc);
            sum += &match_number(number % 10);
        }
        number /= 10;
        acc += 1;
    }
    sum.chars().rev().collect()
}
fn match_number(nchar: i64) -> String {
    match nchar {
        0 => "零".to_string(),
        1 => "壹".to_string(),
        2 => "贰".to_string(),
        3 => "叁".to_owned(),
        4 => "肆".to_string(),
        5 => "伍".to_string(),
        6 => "陆".to_string(),
        7 => "柒".to_string(),
        8 => "捌".to_string(),
        9 => "玖".to_string(),
        _ => panic!("error: unexpected the program runs with an error"),
    }
}

fn match_unit(number: i64) -> String {
    let nchar = number / 4;
    match nchar {
        1 => "万".to_string(),
        2 => "亿".to_string(),
        3 => "兆".to_string(),
        4 => "京".to_string(),
        // 5 => "垓".to_string(),
        // 6 => "秭".to_string(),
        _ => panic!("error: unexpected the program runs with an error"),
    }
}
fn match_unit_number(number: i64) -> String {
    let nchar = number % 4;
    match nchar {
        1 => "拾".to_string(),
        2 => "佰".to_string(),
        3 => "仟".to_string(),
        4 => "".to_string(),
        _ => panic!("error: unexpected the program runs with an error"),
    }
}
fn match_unit_float(number: i64) -> String {
    match number {
        1 => "分".to_string(),
        2 => "角".to_string(),
        _ => panic!("error: unexpected the program runs with an error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_chinese_string() {
        assert_eq!(int_to_chinese_string(0), "零元");
        assert_eq!(int_to_chinese_string(1), "壹元");
        assert_eq!(int_to_chinese_string(10), "壹拾元");
        assert_eq!(int_to_chinese_string(10001), "壹万零壹元");
        assert_eq!(
            int_to_chinese_string(123456789),
            "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖元"
        );
        assert_eq!(
            int_to_chinese_string(123400780),
            "壹亿贰仟叁佰肆拾万零柒佰捌拾元"
        );
        assert_eq!(int_to_chinese_string(100000008), "壹亿零捌元");
        assert_eq!(int_to_chinese_string(100060008), "壹亿零陆万零捌元");
    }

    #[test]
    fn test_float_to_chinese_string() {
        assert_eq!(
            float_to_chinese_string(100060008.12),
            "壹亿零陆万零捌元壹角贰分"
        );
        assert_eq!(float_to_chinese_string(0.32), "叁角贰分");
        assert_eq!(float_to_chinese_string(1.05), "壹元伍分");
        assert_eq!(
            float_to_chinese_string(123456789.00),
            "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖元"
        );
        assert_eq!(
            float_to_chinese_string(123400780.5),
            "壹亿贰仟叁佰肆拾万零柒佰捌拾元伍角"
        );
    }
}

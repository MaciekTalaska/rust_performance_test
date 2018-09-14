
fn return_5() -> u32 {
    return 5;
}

pub fn compute_functional(count: u8) -> u32 {
    let elements = (0..count).collect::<Vec<u8>>().iter()
        .map(|_e| return_5()-1).collect::<Vec<u32>>();
    let value = elements.iter()
        .fold(0, |sum, val| sum * 6 + val);

    value
}

pub fn compute_imperative(count: u8) -> u32 {
    let mut value: u32 = 0;
    for _i in { 0..count } {
        value *= 6;
        value += (return_5()) - 1;
    }
    value
}

//#[cfg(test)]
//mod dices_test {
//    fn max_by_dice_num(dice_num: u8) -> u32 {
//        (6 as u32).pow(dice_num as u32) - 1
//    }
//
//    #[test]
//    fn roll_dices_5_dices_should_not_exceed_7775_i() {
//        for _i in { 0..1000 } {
//            let result = self::super::roll_dices_i(5);
//            assert!(result <= max_by_dice_num(5));
//        }
//    }
//
//    #[test]
//    fn roll_dices_5_dices_should_not_exceed_7775_f() {
//        for _i in { 0..1000 } {
//            let result = self::super::roll_dices_f(5);
//            assert!(result <= max_by_dice_num(5));
//        }
//    }
//}

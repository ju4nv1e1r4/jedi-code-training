pub fn is_palindrome_number(x: i32) -> bool {
    let mut mutable_x = x;
    let mut reversed_x: i32 = 0;
    
    if x < 0 {
        return false;
    }

    if x == 0 {
        return true;
    }

    while mutable_x > 0 {
        let digit = mutable_x % 10;
        reversed_x = reversed_x * 10 + digit;
        mutable_x = mutable_x / 10;
    }

    if reversed_x == x {
        return true;
    } else {
        return false;
    }
}
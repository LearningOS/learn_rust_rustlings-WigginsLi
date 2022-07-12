// iterators4.rs



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    (1..num+1).product()

    // if num <=2 {
    //     num
    // } else {
    //     let mut v : Vec<u64> = vec![0; 10];
    //     v = v.iter()
    //         .enumerate()
    //         .map(|(idx, val)| 
    //             {
    //                 if idx <= 2 {
    //                     idx as u64
    //                 }else {
    //                     v[idx-1] * (idx as u64)
    //                 }
    //             })
    //         .collect::<Vec<u64>>();
            
    //     eprintln!("{:?}", v);
    //     match v.get(num as usize) {
    //         Some(x) => *x,
    //         None => 0
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

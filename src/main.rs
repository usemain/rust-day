fn main() {
    let a = 5;

    println!("a = {}", a);

    let arr = [1, 2, 3];

    println!("arr = {:?}", arr);
    for i in arr {
        println!("i = {}", i);
    }

    let arr2 = (1, 2);
    println!("arr2 = {:?}", arr2);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);
}

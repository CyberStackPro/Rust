// fn main(){
//     let number  =3;
//     if number < 5 {
//         println!("condition was true")
//     }else{
//         println!("condition was false");
//     }
// }
// fn main() {
//     loop {
//         println!("again!");
//     }
// }
fn main() {
    let mut counter = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
}

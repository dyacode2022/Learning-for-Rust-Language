use std::io;

fn main() {
    println!("--------------");

    let mut number = String::new();

    println!("아무 수나 입력하세요.");
    io::stdin().read_line(&mut number).expect("Failed to read line"); //수를 입력받는다.

    let number: u32 = number.trim().parse().expect("Please type a number!"); //입력받은 수의 타입을 string 에서 int 로 변환한다.

    
    if number > 10 {  //if 문으로 10보다 큰지, 작은지 출력한다.
        println!("이 수는 10 보다 큰 수 입니다.");
    } else {
        println!("이 수는 10 보다 작은 수 입니다.");
    }

    println!("--------------");

    let mut nums: u32 = 5;

    while nums != 0 {  //while 문을 사용하여 nums 변수가 0이 아닐 동안 1씩 빼며 출력한다.
        println!("{}!", nums);
        nums-= 1;
    }

    println!("--------------");

    let array = [10, 20, 30, 40, 50];

    for element in array.iter() { //배열에 있는 값을 for 문으로 하나씩 출력한다.
        println!("{}!", element);
    }

    println!("--------------");

    for number in (1..6).rev() {  //for 문의 rev 메서드를 이용하여 1 부터 5 까지 출력한다.
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

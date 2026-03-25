fn main() {
    // 1. 변수
    // let x = 5;
    // println!("{x}");
    // // x = 6; // 에러: 불변 변수에 두 번 값을 할당할 수 없다.
    // println!("{x}");

    // 2. 가변 변수
    // let mut x = 5;
    // println!("{x}");
    // x = 6;
    // println!("{x}");

    // 3. 상수
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 4. 섀도잉
    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2; // 12
    //     println!("{x}");
    // }
    //
    // println!("{x}"); // 6
    //
    // let spaces = "    ";
    // let spaces = spaces.len(); // let 키워드를 사용해 변수를 생성하므로 다른 타입의 값을
    // // 저장할 수 있다.

    // 5. 부동 소수점
    // let x = 2.0; // f64
    //
    // let y: f32 = 3.0; // f32

    // 6. 수치 연산
    // let sum = 5 + 10;
    //
    // let difference = 95.5 - 4.3;
    //
    // let product = 4 * 30;
    //
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    //
    // let remainder = 43 % 5;
    //
    // println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    // 7. 문자
    // let c = 'z';
    // let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    // let heart_eyed_cat = '😻';
    //
    // println!("{c}, {z}, {heart_eyed_cat}");

    // - 복합 타입
    // 8. 튜플
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{x},{y},{z}");
}

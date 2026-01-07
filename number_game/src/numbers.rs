pub fn numbers() {
    // Definieer twee i16 getallen
    let num1: i8 = 16;
    let num2: i8 = 5;

    // Voer basis wiskundige bewerkingen uit
    let result1: i8 = num1 + num2;
    let result2: i8 = num1 - num2;
    let result3: i8 = num1 / num2;
    let num3: i16 = num1 as i16;
    let num4: i16 = num2 as i16;
    let result4: i16 = num3 * num4;
    let num5: i32 = num1 as i32;
    let num6: i32 = num2 as i32;
    let result5: i32 = num5.checked_pow(num6 as u32).unwrap_or(0);

    // Print de resultaten
    println!("Optellen: {result1}");
    println!("Aftrekken: {result2}");
    println!("Delen: {result3}");
    println!("Vermenigvuldigen: {result4}");
    println!("Machten: {result5}");
}
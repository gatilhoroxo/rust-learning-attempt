fn main() {
    println!("Hello, world!");
    print!("Coisas ruins acontecem com pessoas boas");
    print!("!\nolhe bem para mim\n");

    /*
    // ESTOU SEGUINDO O TUTORIAL DO W3SCHOLS.COM/RUST
    // HAHA MUSICAS DO SPOTIFY

    println!("\n Variaveis: ");
    //variáveis imutáveis -----------------------------
    let name = "John";
    let num = 30;
    println!("MY NAME IS: {} OF {} HARDWARE", name, num);

    //variáveis mutáveis -----------------------------
    let mut x = 5;
    print!("x Antes: {}; ", x);
    x = 10;
    println!("x Depois: {}; ", x);

    //TIPOS DE DADOS  --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    println!("\n Tipos de Dados:");

    //declaração implicita -----------------------------
    let num = 5;                // inteiro
    let ponto = 5.99;           // float
    let letra = 'D';            // character
    let is_bool = true;         // boolean
    let text = "OLÁ";           // string
    println!("\n Implicita:{},{},{},{},{}",num,ponto,letra,is_bool,text);

    //declaração explicita -----------------------------
    let nume: i32 = 5;          // inteiro
    let pontoe: f64 = 5.99;     // float
    let letrae: char = 'D';     // character
    let is_boole: bool = true;  // boolean
    let texte: &str = "OLAA";   // STRING
    println!("\n Explicita:{},{},{},{},{}",nume,pontoe,letrae,is_boole,texte);
    
    //constantes --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    //DEVEM TER UM TIPO EXPLICITO 
    const MAXIMO: i32 = 1_000_000;
    println!("\n Constantes: {}", MAXIMO);

    
    //operadores --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    println!("\n Operadores:");
    let add = 5 + 4;
    let sub = 10 - 3;
    let mul = 5 * 3; 
    let div = 9 / 2;
    let rem = 10 % 3;
    
    print!(" Add: {}\n Sub: {}\n Mul: {}\n Div: {}\n Rem: {}\n",add,sub,mul,div,rem);
    
    //operadores de atribuicao -----------------------------
    println!("\n Op. de atribuição:");
    let mut bob = 5;    println!("start: {}",bob);
    bob+=5;             println!("after +=5: {}", bob);
    bob-=2;             println!("after -=2: {}", bob);
    bob*=2;             println!("after +=2: {}", bob);
    bob/=3;             println!("after /=3: {}", bob);
    bob%=4;             println!("after %=4: {}", bob);
    
    //operadores de comparação -----------------------------
    println!("\n Op. de comparação:");
    let a = 5;
    let b = 10;
    println!("5==10: {}", a==b);
    println!("5!=10: {}", a!=b);
    println!("5<10: {}", a<b);
    println!("5>=10: {}", a>=b); 
    
    //operadores lógicos -----------------------------
    println!("\n Op. lógicos:");
    let is_log_in   = true;
    let is_adm      = false;
    println!("User:    {}", is_log_in && !is_adm);
    println!("Access:  {}", is_log_in || is_adm);
    println!("Not log: {}", !is_log_in);
    
    //IF ELSE --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    println!("\n Statements if else:");
    if is_log_in {
        print!("Logado!\n");
    } else if is_adm {
        print!("Eh admin!\n");
    } else {
        print!("Tem como ficar logado... talvez\n");
    }
    let time = 20; 
    let greeting = if time < 18 { "Good day." } else { "Good evening." };
    println!("{}",greeting);

    
    //MATCH --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    println!("\n Statement match:");
    let day = 4;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Friday"),
        5 => println!("Saturday"),
        6 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day."),
    }
    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };
    println!("{}", result);

    
    //LOOP --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    let mut cnt = 1;
    loop {
        println!("hello wolrd!!!");
        if cnt == 3 { break; }
        cnt+=1;
    }
    cnt = 0;
    let rest = loop {
        println!("Hello!");
        if cnt == 3 { break cnt; }
        cnt+=1;
    };
    println!("The loop stopped at: {}", rest);
    
    //WHILE --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    while cnt < 10 {
        if cnt == 7 { cnt+=1; continue; }
        cnt+=1;
    }
    
    //FOR --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    for i in 1..6 {
        println!("i is: {}", i);
    }
    for i in 1..=6 { //INCLUSIVO
    println!("i is: {}", i);
    }

    //FUNCOESS --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    hello();
    
    //STRINGS --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    let text1 = "OI".to_string();
    let text2 = String::from("OI");
    let mut greeting = String::from("OI");
    greeting.push_str(" MUNDO");
    greeting.push('!');
    println!("{}",greeting);
    */

    //concatenando strings
    let s1 = String::from("kk");
    let d2 = String::from("oo");
    let o4 = String::from("fdf");
    //let retaf = format!("{} {} {}",s1,d2,o4);
    let retaf = s1 + &d2 + &o4;
    println!("{}, tam {}",retaf, retaf.len());

    //OWNERSHIP --=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    //...
}

/*
fn hello(){
    println!("Hello from a functionn!");
    greet("jon");
}

fn greet(name: &str){
    println!("Hello, {} {}!", add(2,2), name);
}

fn add(a: i32, b: i32) -> i32 {
    return a+b;
}
*/
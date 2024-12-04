fn main() {
    let x: i8 = 10;
    println!("{}",x);

    let _y: u8 =23;
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}",_y);
    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);



    let byte = b'A';
    let d =2.0;
    let float: f32 = 2.3;
    let t = true;
    let f: bool = false;
    let c = 'c';

    println!("{}",byte);
    println!("{}",d);
    println!("{}",float);
    println!("{}",t);
    println!("{}",f);
    println!("{}",c);


    let m=20;
    let n =3;
    let rem= m%n;
    println!("{}",rem);
}

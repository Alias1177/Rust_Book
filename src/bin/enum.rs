enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String)
}

fn main(){
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    let present =Some(1);
    let absent: Option<i32>=None;
     
    let config_max = Some(3u8);

    if let Some(max) = config_max{
        println!("The maximum is configured to be {}",max);
    }else{
        println!("The maximum is not configured");
    }

    match present{
        Some(i) => println!("{}",i),
        None => println!("None")
    }

    match absent{
        Some(i) => println!("{}",i),
        None => println!("None")
    }

    match four{
        IpAddrKind::V4(a,b,c,d) => println!("{} {} {} {}",a,b,c,d),
        IpAddrKind::V6(_) => todo!()
    }
    match six{
        IpAddrKind::V4(a,b,c,d) => println!("{} {} {} {}",a,b,c,d),
        IpAddrKind::V6(s) => println!("{}",s)
    }
}

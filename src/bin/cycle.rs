fn main(){

let mut num: i128=0;

    let mut n =0;

    let result =loop{
        n+=1;

        if n==10{
            break n*2;
        }
    };
    println!("{}",n);

    println!("{result}");

    loop{
        if num==1000{
            break;
        }
        num+=1;
        //println!("{}",num);
    }

    let mut n:i128=0;
      while n<10 {

        n+=2;

        println!("{}",n);
        };


     let mut count = 0;

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
    


    //while как loop с условием
    let mut nn = 0;

    while nn !=0 {
        nn-=1;
        println!("{}",nn);
    }
    println!("End");

    /* 
    цикл с массивом,он не безопастный тк если мы забудем в цикле поменять условие и сделае index
    условно 4 то у нас будет паника 
    */
    let m =[10,20,30,40,50];
    let mut index = 0;

    while index <5 {
        println!("{}",m[index]);
        index+=1;
    }
    println!("End");

    //безопастный вариант 
    for a in m{
        println!("{}",a);
    }

    //альтернативные переюор не включает последнее число
    //revers
    for i in (1..5).rev(){
        println!("{}",i);
    }
    println!("End");
}

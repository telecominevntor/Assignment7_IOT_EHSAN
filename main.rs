mod rules {
    pub mod order{
        pub mod courts{
            pub fn law(data:String,pending_cases:i32){

                println!(" ");
                println!("{}  : {}",data,pending_cases);
            }
        }
    }
    pub fn forces(){
            let data2= "ARMY, NAVY, AIRFORCE".to_string();
            let soliders = 8899999;
            println!(" ");
            println!("The security agencies are responsible for the internal and external security of the State of the Country  {} and total soliders approx = {}",data2,soliders);




    }
}
use std::io;
mod lib;
use library1;

    fn main() {
        println!(" ");
        println!(" Please enter your unit name");
        let data1=String::from("LAW IS THE BACK BONE OF the country but still some pending cases must be heared on daily basis are ");
        let pending_cases=555;

        let lawyers=String::from("During 2020 Pakistan population is projected ");
        let pending_cases1=211854070;

        let mut data2= String::new();
        io::stdin().read_line(&mut data2).expect("failed to read line");

        let data3=String::from("National Indigenous Law and Justice Framework was developed by the Standing Committee of Attorney-General (SCAG) ");
        let high_courts= 5;


    rules::order::courts::law(data1,pending_cases);
    rules::forces();
    lib::judges::justice::law(lawyers,pending_cases1);
    library1::legislative::majority_leader::law_enforcement(data3,high_courts);


    }

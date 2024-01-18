mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza{
        pub fn lunch(topping:&str)->Pizza{
            Pizza{
                dough: String::from("Reg dough"),
                cheese: String::from("Mozarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_cust{
        fn seat_at_table(){
            println!("Cust seated");
        }
        pub fn take_order(){
            seat_at_table();
            let cust_pizza: super::Pizza= super::Pizza::lunch("Veggies");
            serve_cust(cust_pizza);
        }
        fn serve_cust(cust_pizza:super::Pizza){
            println!("The cust is served mut a reg pizza with {}",cust_pizza.topping);
        }
    }


}


pub fn order_food(){
    crate::restaurant::pizza_order::help_cust::take_order();
}

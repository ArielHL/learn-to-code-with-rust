#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str,
}

fn main() {
    // let from = "New York";
    // let to = "Los Angeles";
    // let plan = TravelPlan { from, to };
    // println!("{:?}", plan);

    let from = String::from ("New York");

    let plan = {
        let to = String::from("Los Angeles");
        let travel_plan = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
    };

    println!("{}", plan);
}

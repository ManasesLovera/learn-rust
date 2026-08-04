mod garden;

fn main() {
    let var = garden::fun();

    println!("{}", var);

    let my_vege = garden::vegetables::Asparagus::constructor("eggplant");

    println!("Veggie name: {}", my_vege.get_name());
}

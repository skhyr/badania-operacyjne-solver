use lib::ui::render::main as render;

fn main() {
    // let equation_system = vec![
    //     Equation(12.0, 4.0, 480.0),
    //     Equation(8.0, 8.0, 640.0),
    //     Equation(1.0, -1.0, 0.0),
    //     Equation(-1.0, 0.0, 0.0),
    //     Equation(0.0, -1.0, 0.0),
    // ];
    // let score_fn = Equation(50.0, 10.0, 0.0);
    // let res = find_optimum(&equation_system, &score_fn);
    // println!("#{:?}", res);
    render().unwrap();
}

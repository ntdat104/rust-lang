mod closure;
mod data_types;
mod flow_and_conditions;
mod functions_and_modules;
mod types_and_variables;
mod variables_and_constants;

fn main() {
    variables_and_constants::run();
    data_types::run();
    types_and_variables::run();
    functions_and_modules::run();
    functions_and_modules::namehelpers::print();
    flow_and_conditions::run();
    flow_and_conditions::run_while();
    flow_and_conditions::run_loop();
    flow_and_conditions::run_for_loop();
    closure::run();
}

mod file_container;
mod typedef_parser;

use regex::Regex;

fn print_init_expr(node: typedef_parser::Node, init_expr: &mut String, result_expr: &mut Vec<String>) {
    let pattern = r#"(\[[\s\t\n]*\d+[\s\t\n]*\])"#;
    let regex = Regex::new(pattern).unwrap();
    let mut arr_size = String::new();
    if let Some(value) = regex.captures(&node.data_type) {
        arr_size.push_str(&(value[1]));
    }
    let new_expr = node.name + &arr_size + ".";
    init_expr.push_str(&new_expr);

    let mut pre_expr = String::new();
    pre_expr.push_str(&init_expr);

    if typedef_parser::is_primivite_type(&(node.data_type)) {
        init_expr.pop();
        init_expr.push_str(" = 0;");
        result_expr.push(init_expr.clone());

        return;
    }

    for below_node in node.below_nodes {
        print_init_expr(below_node, init_expr, result_expr);
        init_expr.clear();
        init_expr.push_str(&pre_expr);
    }
}

// 리팩토링 필요
fn array_post_proc(init_expr: &mut String) {
    let index_variables = [
        "myi",
        "myj",
        "myk",
        "myl",
        "mym",
        "myn",
        "myo",
        "myp",
        "myq",
        "myr",
    ];

    let regex = Regex::new(r#"\[[\s\t\n]*(\d+)[\s\t\n]*\]"#).unwrap();
    let count = regex.find_iter(init_expr).count();
    match count {
        0 => {},
        _ => {
                let mut result = String::new();
                for i in 0..count {
                    let mut num_vector = Vec::new();
                    for caps in regex.captures_iter(init_expr) {
                        let num = caps[1].parse::<i32>().unwrap();
                        num_vector.push(num);
                    }

                    let mut indent_syntax = String::new();
                    for _j in 0..i {
                        indent_syntax.push_str("\t");
                    }
                    let for_sytax = "for (int ".to_string() + index_variables[i] + " = 0; " + index_variables[i] + " < " + &num_vector[i].to_string() +"; "+ index_variables[i]+"++) {\n";
                    result.push_str(&indent_syntax);
                    result.push_str(&for_sytax);
                }

                let mut indent_syntax = String::new();
                for _i in 0..count {
                    indent_syntax.push_str("\t");
                }
                result.push_str(&indent_syntax);

                let mut temp: usize = 0;
                let edited_init_expr = regex.replace_all(&init_expr, |caps: &regex::Captures| {
                    let result = format!("[{}]", index_variables[temp]);
                    temp = temp + 1;
                    result
                });
                result.push_str(&edited_init_expr);
                result.push_str("\n");

                for i in (0..count).rev() {
                    for _j in 0..i {
                        result.push_str("\t");
                    }
                    result.push_str("}\n");
                }

                init_expr.clear();
                init_expr.push_str(&result);
            }
    }


}


fn main() {
    let mut main_c = file_container::File::new(r#".\test-files\main.c"#.to_string());
    let mut source = String::new();
    source.push_str(&main_c.contents);

    let type_h = file_container::File::new(r#".\test-files\type.h"#.to_string());
    let mut header = String::new();
    header.push_str(&type_h.contents);

    let typedef_exprs = typedef_parser::extract_typedef(&header);
    let mut typedef_database = Vec::<typedef_parser::Typedef>::new();
    for typedef_expr in typedef_exprs {
        typedef_database.push(typedef_parser::lexicalize_typedef(typedef_expr));
    }

    //main source parsing
    let result_tree = typedef_parser::parse(&typedef_database, "targetStruct", "testname");
    
    let mut temp = String::new();
    let mut init_exprs = Vec::<String>::new();

    print_init_expr(result_tree, &mut temp, &mut init_exprs);

    for init_expr in &mut init_exprs {
        array_post_proc(init_expr);
        println!("{}",init_expr);
    }
}

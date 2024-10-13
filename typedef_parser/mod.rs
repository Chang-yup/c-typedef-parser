#![allow(dead_code)]

use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


/// typedef array 정규표현식
/// typedef[\s\n\t]+struct[\s\n\t]+(\w+)[\s\n\t]*\{([\s\S]*?)\}[\s\t\n]+(\w+)?;
/// $1: dataType, $2: alias, $3: size
/// 
/// typedef 일반 정규표현식
/// typedef[\s\t\n]*\S+[\s\t\n]*\S+[\s\n\t]*;
/// $1: dataType, $2: alais
#[derive(Debug, EnumIter, PartialEq)]
pub enum RawTypedef {
    Struct(String),
    Array(String),
    Normal(String),
}
impl RawTypedef {
    fn get_regex(&self) -> &str {
        match self {
            RawTypedef::Struct(..) => r#"typedef[\s\n\t]+struct[\s\n\t]+(\w+)[\s\n\t]*\{([\s\S]*?)\}[\s\t\n]+(\w+)?;"#,
            RawTypedef::Array(..)  => r#"typedef[\s\t\n]+(\w+)[\s\t\n]+(\w+)[\s\n\t]*\[[\s\n\t]*(\d+)[\s\n\t]*\][\s\n\t]*;"#,
            RawTypedef::Normal(..) => r#"typedef[\s\t\n]+(\w+)[\s\t\n]+(\w+)[\s\n\t]*;"#,
        }
    }
}


/// 변수 선언문 해석용 enum
/// Pointer 정규표현식
/// ^[\s\t\n]*(\w+)[\s\t\n]*\*[\s\t\n]*(\w+);
/// $1: dataType, $2: variableName
/// 
/// Array 정규표현식
/// ^[\s\t\n]*(\w+)[\s\t\n]*(\w+)[\s\t\n]*\[[\s\t\n]*\d+[\s\t\n]*\][\s\t\n]*;
/// $1: dataType, $2: variableName, $3: size
/// 
/// 일반 정규표현식
/// ^[\s\t\n]*(\w+)[\s\t\n]*(\w+);
/// $1: dataType, $2: variableName
#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum Declaration {
    Pointer{
        name:String,
        data_type:String,
    },
    Array{
        name:String,
        data_type:String,
        size:usize
    },
    Normal{
        name:String,
        data_type:String,
    },
}
impl Declaration {
    fn get_regex(&self) -> &str {
        match self {
            Declaration::Pointer{..} => r#"(\w+)[\s\t\n]*\*[\s\t\n]*(\w+);"#,
            Declaration::Array{..} => r#"(\w+)[\s\t\n]+(\w+)[\s\t\n]*\[[\s\t\n]*(\d+)[\s\t\n]*\][\s\t\n]*;"#,
            Declaration::Normal{..} => r#"(\w+)[\s\t\n]+(\w+);"#,
        }
    }
    fn extract_declaration (code: &str) -> Vec<Declaration> {
        let mut result = Vec::<Declaration>::new();
        
        

        for decl in Declaration::iter() {
            let pattern = decl.get_regex();
            let regex = Regex::new(pattern).unwrap();
            for captured_str in regex.captures_iter(code) {
                match decl {
                    Declaration::Pointer{..} => result.push(Declaration::Pointer { name: captured_str[2].to_string(), data_type: captured_str[1].to_string() }),
                    Declaration::Array{..} => result.push(Declaration::Array { name: captured_str[2].to_string(), data_type: captured_str[1].to_string(), size: captured_str[3].parse().unwrap() }),
                    Declaration::Normal{..} => result.push(Declaration::Normal { name: captured_str[2].to_string(), data_type: captured_str[1].to_string() }),
                }
            }
        }
    
        result
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Typedef {
    Struct{
        alias: String,
        data_type: String,
        elements: Vec<Declaration>
    },
    Array{
        alias: String,
        data_type: String,
        size: usize,
    },
    Normal{
        alias: String,
        data_type: String,
    },
}
pub fn extract_typedef(code: &str) -> Vec<RawTypedef> {
    let mut result = Vec::<RawTypedef>::new();

    for typedef in RawTypedef::iter() {
        let pattern = typedef.get_regex();
        let regex = Regex::new(pattern).unwrap();
        for captured_str in regex.find_iter(code) {
            match typedef {
                RawTypedef::Struct(..) => result.push(RawTypedef::Struct(captured_str.as_str().to_string())),
                RawTypedef::Array(..) => result.push(RawTypedef::Array(captured_str.as_str().to_string())),
                RawTypedef::Normal(..) => result.push(RawTypedef::Normal(captured_str.as_str().to_string())),
            }
        }
    }

    result
}

pub fn lexicalize_typedef(raw_typedef: RawTypedef) -> Typedef {
    match raw_typedef {
        ref handler @ RawTypedef::Struct(ref typedef_expr) => {
            let pattern = handler.get_regex();
            let regex = Regex::new(pattern).unwrap();

            match regex.captures(typedef_expr) {
                Some(captured) => {
                    let elements = Declaration::extract_declaration(&(captured[2].to_string()));
                    Typedef::Struct { 
                        alias: captured[3].to_string(),
                        data_type: captured[1].to_string(), 
                        elements: elements,
                    }
                },
                None => {
                    panic!("Error capturing {} in {}", pattern, typedef_expr);
                }
            }
        },
        ref handler @ RawTypedef::Array(ref typedef_expr) => {
            let pattern = handler.get_regex();
            let regex = Regex::new(pattern).unwrap();

            match regex.captures(&typedef_expr) {
                Some(captured) => {
                    Typedef::Array { 
                        alias: captured[2].to_string(), 
                        data_type: captured[1].to_string(), 
                        size: captured[3].parse().unwrap()
                    }
                },
                None => {
                    panic!("Error capturing {} in {}", pattern, typedef_expr);
                }
            }
        },
        ref handler @ RawTypedef::Normal(ref typedef_expr) => {
            let pattern = handler.get_regex();
            let regex = Regex::new(pattern).unwrap();

            match regex.captures(&typedef_expr) {
                Some(captured) => {
                    Typedef::Normal { 
                        alias: captured[2].to_string(), 
                        data_type: captured[1].to_string()
                    }
                        
                },
                None => {
                    panic!("Error capturing {} in {}", pattern, typedef_expr);
                }
            }
        },
    }

}

fn find_by_alias<'a, 'b>(typedefs: &'a Vec<Typedef>, target_alias: &'b str) -> &'a Typedef {
    let mut result = Vec::<&Typedef>::new();
    
    for typedef in typedefs {
        let alias = match typedef {
            Typedef::Struct { alias, .. } => alias,
            Typedef::Array { alias, .. } => alias,
            Typedef::Normal { alias, .. } => alias,
        };

        if alias == target_alias {
            result.push(typedef);
        }
    }

    match result.len() {
        1 => result[0],
        0 => panic!("{} is not found.", target_alias),
        _ => panic!("{} is found more than once.", target_alias),
    }
}

pub fn is_primivite_type(type_name: &str) -> bool {
    let primitive_type_list = [
        "boolean",
        "uint8",
        "uint16",
        "uint32",
        "int8",
        "int16",
        "int32",
        "sint8",
        "sint16",
        "sint32"];

    for primitive_type in primitive_type_list {
        if type_name == primitive_type {return true;}
    }

    false
}

/// 함수 2
/// parse
/// 원하는 dataType과 db(datatype enum vector)를 받으면 tree 구조로 dataType을 분해해주는 함수
/// struct node {}
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub data_type: String,
    pub below_nodes: Vec<Node>,
}

pub fn parse(typedefs: &Vec<Typedef>, typedef_alias:&str, element_name: &str) -> Node{
    // 1. 빈 노드를 만든다.
    let mut result = Node { name: element_name.to_string(), data_type: String::new(), below_nodes: Vec::<Node>::new() };
    // 2. target_struct_type_name이 primitive type인지 검사한다. (맞다면 below_nodes가 없는 상태로 반환한다)
    match is_primivite_type(typedef_alias) {
        true => {
            result.data_type = typedef_alias.to_string();
        },
        false => {
            let typedef = find_by_alias(typedefs, typedef_alias);
            match typedef {
                Typedef::Struct { alias, elements, .. } => {
                    result.data_type = alias.to_string();
                    for element in elements {
                        match element {
                            Declaration::Normal { name, data_type } => {
                                result.below_nodes.push(parse(typedefs, data_type, name));
                            },
                            Declaration::Pointer { .. } => {
                                panic!("Pointer declaration in struct is not supported yet.");
                            },
                            Declaration::Array {..} => {
                                panic!("Array declaration in struct is not supported yet. (It must be alias by typedef)");
                            }
                        }
                    }
                },
                Typedef::Array { data_type, size, .. } => {
                    let mut temp_node = parse(typedefs, data_type, element_name);
                    let new_data_type = temp_node.data_type.to_string() + "[" + &(size.to_string()) + "]";
                    temp_node.data_type = new_data_type;
                    result = temp_node;
                },
                Typedef::Normal { data_type, .. } => {
                    result = parse(typedefs,data_type, element_name);
                }
            }
        }
    }
    // 4. db에 있다면 Vec<Node>를 만들어 하위 노드에 대해 parse함수를 반복한 후 push한다.
    // 5. db에 없다면 오류를 반환한다. 
    return result
}
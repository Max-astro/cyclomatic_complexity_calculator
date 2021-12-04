use tree_sitter::{Node, Parser, Tree};
use tree_sitter_python;
pub struct FunctionCC {
    cnt: usize,
}

impl FunctionCC {
    pub fn count_cc(func_root: Node) -> usize {
        let mut cc = FunctionCC { cnt: 1 };
        cc.travers(func_root);
        cc.cnt
    }

    fn travers(&mut self, node: Node) {
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            match child.kind() {
                "for_statement" => self.cnt += 1,
                "if_statement" => self.cnt += 1,
                "while_statement" => self.cnt += 1,
                "try_statement" => self.cnt += 1,
                "except_clause" => self.cnt += 1,
                "finally_clause" => self.cnt += 1,
                "elif_clause" => self.cnt += 1,
                "function_definition" => self.cnt += 1,
                _ => {}
            };
            self.travers(child);
        }
    }
}

pub struct AST<'a> {
    src: &'a str,
    tree: Tree,
}

impl<'a> AST<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_python::language())
            .expect("Error loading Python grammar");
        let tree = parser.parse(src, None).unwrap();
        // let root = tree.root_node();
        AST { src, tree }
    }

    fn travers_ast(&self) -> Vec<(String, usize)> {
        let mut res = vec![];
        let root = self.tree.root_node();
        for i in 0..root.child_count() {
            let child = root.child(i).unwrap();
            match child.kind() {
                "class_definition" => {
                    self.process_class(child, &mut res);
                }
                "function_definition" => {
                    res.push(self.process_function(child, None));
                }
                _ => {}
            }
        }
        res
    }

    fn process_class(&self, node: Node, res: &mut Vec<(String, usize)>) {
        let name_node = node.child(1).unwrap();
        assert_eq!(name_node.kind(), "identifier");
        let class_name = &self.src[name_node.byte_range()];

        // println!("{}:", class_name);
        // Self::dbg_ast(node);
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == "block" {
                for i in 0..child.child_count() {
                    let child = child.child(i).unwrap();
                    if child.kind() == "function_definition" {
                        res.push(self.process_function(child, Some(class_name)));
                    }
                }
            }
        }
    }

    fn process_function(&self, node: Node, class: Option<&str>) -> (String, usize) {
        let name_node = node.child(1).unwrap();
        assert_eq!(name_node.kind(), "identifier");
        let cc = FunctionCC::count_cc(node);
        let fn_name = &self.src[name_node.byte_range()];
        let full_name = if let Some(class) = class {
            format!("{}.{}", class, fn_name)
        } else {
            fn_name.to_string()
        };
        (full_name, cc)
    }

    fn dbg_ast(node: Node) {
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            println!("{:?}", child);
        }
    }
}

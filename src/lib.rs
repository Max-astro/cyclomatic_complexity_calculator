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

    pub fn travers_ast(&self) -> Vec<(String, usize)> {
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

    #[allow(dead_code)]
    fn dbg_ast(node: Node) {
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            println!("{:?}", child);
        }
    }
}

use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use walkdir::WalkDir;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

type CcResult = Vec<(String, usize)>;
pub struct FileCcCalculator(HashMap<String, CcResult>);

impl FileCcCalculator {
    fn single_file_cc(filepath: PathBuf) -> (String, CcResult) {
        let mut src = String::new();
        let mut reader = BufReader::new(File::open(&filepath).unwrap());
        let _ = reader.read_to_string(&mut src);

        let ast = AST::new(&src);
        let cc = ast.travers_ast();
        let file_name = filepath.to_str().map(|f| f.to_string()).unwrap();
        (file_name, cc)
    }

    pub fn process_files(path: &str) -> Self {
        let pool = ThreadPool::new(4);

        let (tx, rx) = channel();

        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().unwrap_or_default() == "py")
        {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let cc = Self::single_file_cc(path);
                tx.send(cc).expect("Could not send data!");
            });
        }

        drop(tx); // manually close channel

        let mut cc_results = HashMap::new();
        for (file_name, result) in rx.iter() {
            cc_results.insert(file_name, result);
        }
        Self(cc_results)
    }

    pub fn display(&self) -> String {
        let mut string = String::new();
        let FileCcCalculator(map) = self;
        for (file, result) in map {
            string.push_str(&format!("Functions in file \"{}\":\n", file));
            for (function, cnt) in result {
                string.push_str(&format!("  '{}' {}\n", function, cnt));
            }
            string.push('\n');
        }
        string
    }
}

use pyo3::prelude::*;
use std::fs::File;

#[pyfunction]
fn calc_py_cc(dir: String) -> CcResult {
    let file_dir = File::open(&dir).unwrap();
    let mut src = String::new();
    let mut reader = std::io::BufReader::new(file_dir);
    let _ = reader.read_to_string(&mut src);

    let ast = AST::new(&src);
    ast.travers_ast()
}

#[pyfunction]
fn calc_py_files_cc(dir: String) -> HashMap<String, CcResult> {
    let FileCcCalculator(res) = FileCcCalculator::process_files(&dir);
    res
}

#[pyfunction]
fn show_py_files_cc(dir: String) -> String {
    let string = FileCcCalculator::process_files(&dir).display();
    string
}

#[pymodule]
fn py_cyclo_complexity(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_py_cc, m)?)?;
    m.add_function(wrap_pyfunction!(calc_py_files_cc, m)?)?;
    m.add_function(wrap_pyfunction!(show_py_files_cc, m)?)?;

    Ok(())
}

/* Tests */

#[test]
fn test1() {
    let file_dir = File::open("./testcase/t1.py").unwrap();
    let mut src = String::new();
    let mut reader = std::io::BufReader::new(file_dir);
    let _ = reader.read_to_string(&mut src);

    let ast = AST::new(&src);
    let functions = ast.travers_ast();
    println!("{:?}", functions);
}

#[test]
fn test2() {
    let file_dir = File::open("./testcase/py_grammer.py").unwrap();
    let mut src = String::new();
    let mut reader = std::io::BufReader::new(file_dir);
    let _ = reader.read_to_string(&mut src);

    let ast = AST::new(&src);
    let functions = ast.travers_ast();
    println!("{:?}", functions);
}

#[test]
fn multi_thread_test() {
    let a = FileCcCalculator::process_files("./testcase/");
    println!("{}", a.display());
}

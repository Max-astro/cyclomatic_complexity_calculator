# A python source code cyclomatic complexity calculator

## Installation

```bash
$ pip install py-cyclo-complexity
```

## Usage
```python
import py_cyclo_complexity

# return functions cyclomatic complexity in one single source file
single_src_cc = py_cyclo_complexity.calc_py_cc("./testcase/t1.py")

# recursively and concurrently process all python source file under the given directory
# return a dictonary contains all functions cyclomatic complexity in each file
cc_dict = py_cyclo_complexity.calc_py_files_cc("./testcase/")

# print the calculation results of all source files
print(py_cyclo_complexity.show_py_files_cc("/Users/max/OneDrive/Codes/rust/cyclo_complexity/testcase/"))

```

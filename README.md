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

# return a dictonary contains all functions cyclomatic complexity in each file
cc_dict = py_cyclo_complexity.calc_py_files_cc("./testcase/")

```

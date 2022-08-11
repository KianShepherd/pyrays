py_lines = []
with open('pytest.txt', 'r') as f:
    py_lines = f.readlines()

rs_lines = []
with open('paulin.txt', 'r') as f:
    rs_lines = f.readlines()

py_line = py_lines[-3].split()
rs_line = rs_lines[-1].split()

py = (int(py_line[1]), int(py_line[1]) - int(py_line[2]))
rs = (int(rs_line[2].split('/')[1]), int(rs_line[2].split('/')[0]))
total_cov = 100 * ((py[1] + rs[1]) / (py[0] + rs[0]))

print('=============================================================================')

for line in py_lines:
    print(line)

print('=============================================================================')

for line in rs_lines:
    print(line)

print('=============================================================================')

print(f'{total_cov:.2f}%')

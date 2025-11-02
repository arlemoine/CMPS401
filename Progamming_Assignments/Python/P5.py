import sys
from zipfile import ZipFile

zip_path = sys.argv[1]
output_file = "output.txt"
all_name_lists = []
common_names = set()

with ZipFile(zip_path) as zf:
    files = []
    for info in zf.infolist():
        if info.is_dir():
            continue
        file = info.filename  
        base = file.rsplit('/', 1)[-1]
        if base.startswith('.') or base.lower() in {'thumbs.db', 'desktop.ini'}:
            continue
        files.append(file)

    for file in files:
        with zf.open(file, 'r') as input_file:
            name_list = input_file.read().decode('utf-8', errors='ignore').splitlines()
            all_name_lists.append(name_list)

for name in all_name_lists[0]:
    if all(name in other_list for other_list in all_name_lists):
        common_names.add(name)

with open(output_file, "w") as output_file:
    for name in common_names:
        print(name)
        output_file.write(name + "\n")

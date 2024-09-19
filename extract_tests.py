import os
import re
import csv

# Input test file containing the unit tests
input_file = 'extract_tests.rs'  # Replace with your actual input file

# Output directories for input and output .rs files
input_dir = 'input'
output_dir = 'correct_output'
os.makedirs(input_dir, exist_ok=True)
os.makedirs(output_dir, exist_ok=True)

# CSV file to store the cursor positions
csv_file = 'test_info.csv'

# Regex patterns for extracting the test name and raw strings
test_name_pattern = re.compile(r'fn (\w+)\(')
raw_string_pattern = re.compile(r'r#"(.*?)"#', re.DOTALL)

# Function to find cursor positions ($0) in the code
def find_cursor_positions(code):
    cursor_positions = []
    lines = code.splitlines()
    for line_num, line in enumerate(lines, start=1):
        for match in re.finditer(r'\$0', line):
            cursor_positions.append((line_num, match.start() + 1))  # Columns are 1-based
    return cursor_positions

# Function to process each test and generate files and CSV entries
def process_tests(input_file, csv_file):
    with open(input_file, 'r') as file, open(csv_file, 'w', newline='') as csvfile:
        csv_writer = csv.writer(csvfile)
        csv_writer.writerow(['Test name', 'Input cursor 1 location', 'Input cursor 2 location'])

        content = file.read()
        test_blocks = content.split('#[test]')

        for test_block in test_blocks[1:]:  # Skip the first element since it's before the first test
            test_name_match = test_name_pattern.search(test_block)
            raw_strings = raw_string_pattern.findall(test_block)

            if test_name_match and len(raw_strings) == 2:
                test_name = test_name_match.group(1)
                input_code = raw_strings[0].strip()  # Remove leading and trailing whitespace
                output_code = raw_strings[1].strip()  # Remove leading and trailing whitespace

                # Find cursor positions in the input code
                input_cursors = find_cursor_positions(input_code)

                # Write input and output files
                input_code_cleaned = input_code.replace('$0', '')  # Remove cursor markers for output
                output_code_cleaned = output_code.replace('$0', '')

                with open(os.path.join(input_dir, f'{test_name}.rs'), 'w') as infile:
                    infile.write(input_code_cleaned)
                with open(os.path.join(output_dir, f'{test_name}.rs'), 'w') as outfile:
                    outfile.write(output_code_cleaned)

                # Write cursor information to CSV
                if len(input_cursors) >= 2:
                    csv_writer.writerow([
                        test_name,
                        f'Line {input_cursors[0][0]}, Column {input_cursors[0][1]}',
                        f'Line {input_cursors[1][0]}, Column {input_cursors[1][1]}'
                    ])
                elif len(input_cursors) == 1:
                    csv_writer.writerow([
                        test_name,
                        f'Line {input_cursors[0][0]}, Column {input_cursors[0][1]}',
                        'N/A'
                    ])
                else:
                    csv_writer.writerow([test_name, 'N/A', 'N/A'])

# Run the script
process_tests(input_file, csv_file)
print(f"Processing complete. Files are saved in '{input_dir}' and '{output_dir}', and the CSV file is '{csv_file}'.")
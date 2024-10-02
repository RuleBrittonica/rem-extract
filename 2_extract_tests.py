import os
import re
import csv

# Input test file containing the unit tests
input_file = '0_extract_tests.rs'  # Replace with your actual input file

# Output directories for input and output .rs files
input_dir = 'input'
output_dir = 'correct_output'
os.makedirs(input_dir, exist_ok=True)
os.makedirs(output_dir, exist_ok=True)

# CSV file to store the cursor positions
csv_file = '0_test_info.csv'

# Regex patterns for extracting the test name and raw strings
test_name_pattern = re.compile(r'fn (\w+)\(')
raw_string_pattern = re.compile(r'r#"(.*?)"#', re.DOTALL)

# Function to find cursor positions ($0) in the code
def find_cursor_positions(code):
    cursor_positions = []

    # Find the first occurrence of $0
    first_pos = code.find('$0')
    if first_pos != -1:
        cursor_positions.append(first_pos)  # Store the position of the first cursor

        # Remove the first occurrence of $0
        code = code[:first_pos] + code[first_pos + 2:]  # Remove '$0'

        # Find the next occurrence of $0 in the modified code
        second_pos = code.find('$0')
        if second_pos != -1:
            cursor_positions.append(second_pos)  # Adjust position for the removal of the first '$0'

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

                # Write input and output files after removing the first cursor
                if len(input_cursors) >= 2:
                    input_code_cleaned = input_code.replace('$0', '')
                    output_code_cleaned = output_code.replace('$0', '')

                    with open(os.path.join(input_dir, f'{test_name}.rs'), 'w') as infile:
                        infile.write(input_code_cleaned)
                    with open(os.path.join(output_dir, f'{test_name}.rs'), 'w') as outfile:
                        outfile.write(output_code_cleaned)

                    # Write character-based cursor information to CSV
                    csv_writer.writerow([test_name, input_cursors[0] - 1, input_cursors[1] + 1])

                elif len(input_cursors) == 1:
                    input_code_cleaned = input_code.replace('$0', '')  # Only one cursor to remove
                    output_code_cleaned = output_code.replace('$0', '')

                    with open(os.path.join(input_dir, f'{test_name}.rs'), 'w') as infile:
                        infile.write(input_code_cleaned)
                    with open(os.path.join(output_dir, f'{test_name}.rs'), 'w') as outfile:
                        outfile.write(output_code_cleaned)

                    # Write single cursor position to CSV
                    csv_writer.writerow([test_name, input_cursors[0], 'N/A'])
                else:
                    # No cursor found
                    csv_writer.writerow([test_name, 'N/A', 'N/A'])

# Run the script
process_tests(input_file, csv_file)
print(f"Processing complete. Files are saved in '{input_dir}' and '{output_dir}', and the CSV file is '{csv_file}'.")

# Open up the CSV file, and sort the rows alphabetically by the test name
import pandas as pd
df = pd.read_csv(csv_file)
df = df.sort_values(by='Test name')
df.to_csv(csv_file, index=False)
print(f"CSV file sorted alphabetically by test name.")
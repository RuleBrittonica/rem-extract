import csv

# Input CSV file containing the test details
csv_file = 'test_info.csv'

# Output Rust file
rust_file = 'src/test_details.rs'

# Function to create the Rust file from the CSV data
def create_rust_file(csv_file, rust_file):
    with open(csv_file, 'r') as csvfile, open(rust_file, 'w') as rustfile:
        csv_reader = csv.DictReader(csvfile)

        rustfile.write("use lazy_static::lazy_static;\n")
        rustfile.write("use crate::test::TestFile;\n\n")
        rustfile.write("lazy_static! {\n")
        rustfile.write("    pub static ref TEST_FILES: Vec<TestFile<'static>> = vec![\n")

        for row in csv_reader:
            test_name = row['Test name']
            input_cursor_1 = row['Input cursor 1 location']
            input_cursor_2 = row['Input cursor 2 location']

            # Parse line numbers from the cursor positions
            start_line = int(input_cursor_1.split(',')[0].split()[1]) if input_cursor_1 != 'N/A' else 0
            end_line = int(input_cursor_2.split(',')[0].split()[1]) if input_cursor_2 != 'N/A' else start_line

            rustfile.write(f'        TestFile {{\n')
            rustfile.write(f'            input_file: "{test_name}.rs",\n')
            rustfile.write(f'            start_line: {start_line},\n')
            rustfile.write(f'            end_line: {end_line},\n')
            rustfile.write(f'        }},\n')

        rustfile.write("    ];\n")
        rustfile.write("}\n")

# Run the script
create_rust_file(csv_file, rust_file)
print(f"Rust file '{rust_file}' has been created.")

import os
import subprocess

# Directory containing the input Rust files
input_directory = 'input'

# Check if the input directory exists
if not os.path.exists(input_directory):
    print(f"The directory '{input_directory}' does not exist.")
    exit(1)

# Iterate over each file in the input directory
for filename in os.listdir(input_directory):
    if filename.endswith('.rs'):
        # Extract the base name without extension
        project_name = os.path.splitext(filename)[0]

        # Define the path for the new project
        project_path = os.path.join(os.getcwd(), project_name)

        # Create a new Rust project
        subprocess.run(['cargo', 'new', project_name])

        # Define the path for the main.rs file in the new project
        main_rs_path = os.path.join(project_path, 'src', 'main.rs')

        # Define the path for the original .rs file
        original_rs_path = os.path.join(input_directory, filename)

        # Copy the contents of the original .rs file to main.rs
        with open(original_rs_path, 'r') as original_file:
            code = original_file.read()

        with open(main_rs_path, 'w') as main_file:
            main_file.write(code)

        print(f"Created project '{project_name}' with main.rs from '{filename}'")

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
        project_path = os.path.join(input_directory, project_name)

        # Create a new Rust project
        subprocess.run(['cargo', 'new', '--bin', project_name], cwd=input_directory)

        # Define the path for the main.rs file in the new project
        main_rs_path = os.path.join(project_path, 'src', 'main.rs')

        # Define the path for the original .rs file
        original_rs_path = os.path.join(input_directory, filename)

        # Copy the contents of the original .rs file to main.rs
        with open(original_rs_path, 'r') as original_file:
            code = original_file.read()

        # Write the original code to main.rs
        with open(main_rs_path, 'w') as main_file:
            main_file.write(code)

        # Clear the contents of main.rs to remove the default `fn main`
        with open(main_rs_path, 'w') as main_file:
            main_file.write("")  # Clear the file

        # Write the original code to main.rs again
        with open(main_rs_path, 'w') as main_file:
            main_file.write(code)

        # Delete the original .rs file
        os.remove(original_rs_path)

        print(f"Created project '{project_name}' in '{input_directory}' and deleted original file '{filename}'")

# For every project in the input directory, go to the main.rs file and add a
# main function at the end of the file (this main function does nothing, but it
# satisfies the Rust compiler)
for folder in os.listdir(input_directory):
    if os.path.isdir(os.path.join(input_directory, folder)):
        main_rs_path = os.path.join(input_directory, folder, 'src', 'main.rs')
        with open(main_rs_path, 'a') as main_file:
            main_file.write("\n\nfn main() {\n\n}\n")

        # Run rustfmt to format the code
        subprocess.run(['rustfmt', "--edition 2021", main_rs_path])
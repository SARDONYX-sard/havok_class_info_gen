import os
import re


def extract_comments_from_file(file_path: str):
    with open(file_path, "r") as file:
        text = file.read()
        if comments_match := re.search(
            r"namespace\s+HKX2\s*\{([^}]*)public\s+partial\s+class", text, re.DOTALL
        ):
            return comments_match[1].strip()
        else:
            return ""


def extract_comments_from_directory(input_directory: str, output_directory: str):
    if not os.path.exists(output_directory):
        os.makedirs(output_directory)

    files = os.listdir(input_directory)
    for file in files:
        file_path = os.path.join(input_directory, file)
        if os.path.isfile(file_path) and file_path.endswith(".cs"):
            if comments := extract_comments_from_file(file_path):
                output_file_path = os.path.join(
                    output_directory, f"{os.path.splitext(file)[0]}.txt"
                )
                with open(output_file_path, "w") as output_file:
                    output_file.write(comments)


output_directory = "./hkx2lib"
input_directory = "HKX2-Library/HKX2/Autogen"
extract_comments_from_directory(input_directory, output_directory)

input_directory = "HKX2/Manual"
extract_comments_from_directory(input_directory, output_directory)

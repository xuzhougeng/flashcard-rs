#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Update main.rs with generated ASCII art
"""

import re
import os

# Get the script directory and project root
script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(script_dir)

# Read the generated ASCII art
with open(os.path.join(script_dir, "hiragana_ascii_art.txt"), "r", encoding="utf-8") as f:
    hiragana_cases = f.read()

with open(os.path.join(script_dir, "katakana_ascii_art.txt"), "r", encoding="utf-8") as f:
    katakana_cases = f.read()

# Read the main.rs file
with open(os.path.join(project_root, "src", "main.rs"), "r", encoding="utf-8") as f:
    content = f.read()

# Find and replace the get_ascii_art function
hiragana_pattern = r'(fn get_ascii_art\(character: &str\) -> Vec<String> \{\s*match character \{)(.*?)(        // Default:.*?\n        \}.*?\n    \}\n\})'
hiragana_replacement = r'\1\n' + hiragana_cases + r'\n\3'

content = re.sub(hiragana_pattern, hiragana_replacement, content, flags=re.DOTALL)

# Find and replace the get_katakana_ascii_art function
katakana_pattern = r'(fn get_katakana_ascii_art\(character: &str\) -> Vec<String> \{\s*match character \{)(.*?)(        // Default.*?\n        \}.*?\n    \}\n\})'
katakana_replacement = r'\1\n' + katakana_cases + r'\n\3'

content = re.sub(katakana_pattern, katakana_replacement, content, flags=re.DOTALL)

# Write back
with open(os.path.join(project_root, "src", "main.rs"), "w", encoding="utf-8") as f:
    f.write(content)

print("Successfully updated src/main.rs with generated ASCII art!")

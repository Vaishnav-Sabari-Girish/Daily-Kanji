import json
from pathlib import Path
import pykakasi

# 1. Initialize the Japanese converter
kks = pykakasi.kakasi()

def convert_to_hiragana(text):
    # kks.convert breaks the sentence into chunks and provides readings
    result = kks.convert(text)
    # Extract the 'hira' (hiragana) string from each chunk and join them
    hiragana_text = "".join([item['hira'] for item in result])
    return hiragana_text

def process_file(filepath: Path):
    # pathlib makes checking existence super clean
    if not filepath.exists():
        print(f"Skipping {filepath} (Not found)")
        return

    print(f"Processing {filepath}...")
    
    # Read the existing JSON
    with filepath.open('r', encoding='utf-8') as f:
        data = json.load(f)

    # Add the 'full_kana' field to every question
    for q in data.get('questions', []):
        if 'sentence' in q:
            q['full_kana'] = convert_to_hiragana(q['sentence'])

    # Write the updated JSON back to the file
    with filepath.open('w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
        
    print(f"✅ Successfully updated {filepath}!")

# 2. Run the script on your asset files
if __name__ == "__main__":
    # Define the base directory and file paths using pathlib
    assets_dir = Path("assets")
    assets = [
        assets_dir / "n5.json", 
        assets_dir / "n4.json", 
        assets_dir / "n3.json"
    ]
    
    print("Starting Kanji to Hiragana conversion...")
    for asset in assets:
        process_file(asset)
    print("All done!")

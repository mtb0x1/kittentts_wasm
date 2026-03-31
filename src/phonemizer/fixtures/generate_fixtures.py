import os
import sys
import json
import numpy as np

# Add the Python project to sys.path
sys.path.append(os.path.join(os.getcwd(), "KittenTTS_python"))

from kittentts.onnx_model import KittenTTS_1_Onnx, basic_english_tokenize

def generate_fixtures():
    # We need dummy paths or real paths for model/voices if we just want the phonemizer/cleaner
    # Actually, we can just instantiate the parts we need.
    import phonemizer
    from kittentts.onnx_model import TextCleaner
    from kittentts.preprocess import TextPreprocessor

    # The model uses these specific settings
    espeak_phonemizer = phonemizer.backend.EspeakBackend(
        language="en-us", preserve_punctuation=True, with_stress=True
    )
    text_cleaner = TextCleaner()
    preprocessor = TextPreprocessor(remove_punctuation=False)

    test_sentences = [
        # List 1
        "The birch canoe slid on the smooth planks.",
        "Glue the sheet to the dark blue background.",
        "It's easy to tell the depth of a well.",
        "These days a chicken leg is a rare dish.",
        "Rice is often served in round bowls.",
        "The juice of lemons makes fine punch.",
        "The box was thrown beside the parked truck.",
        "The hogs were fed chopped corn and garbage.",
        "Four hours of steady work faced us.",
        "A large size in stockings is hard to sell.",
        # List 2
        "The boy was there when the sun rose.",
        "A rod is used to catch pink salmon.",
        "The source of the huge river is the clear spring.",
        "Kick the ball straight and follow through.",
        "Help the woman get back to her feet.",
        "A pot of tea helps to pass the evening.",
        "Smoky fires lack flame and heat.",
        "The soft cushion broke the man's fall.",
        "The salt breeze came across from the sea.",
        "The girl at the booth sold fifty bonds.",
        # List 3
        "The small pup gnawed a hole in the sock.",
        "The fish twisted and turned on the bent hook.",
        "Press the pants and sew a button on the vest.",
        "The swan dive was far short of perfect.",
        "The beauty of the view stunned the young boy.",
        "Two blue fish swam in the tank.",
        "Her purse was full of useless trash.",
        "The colt reared and threw the tall rider.",
        "It snowed, rained, and hailed the same morning.",
        "Read verse out loud for pleasure.",
        # List 4
        "Hoist the load to your left shoulder.",
        "Take the winding path to reach the lake.",
        "Note closely the size of the gas tank.",
        "Wipe the grease off his dirty face.",
        "Mend the coat before you go out.",
        "The wrist was badly strained and hung limp.",
        "The stray cat gave birth to kittens.",
        "The young girl gave no clear response.",
        "The meal was cooked before the bell rang.",
        "What joy there is in living.",
        # List 5
        "A king ruled the state in the early days.",
        "The ship was torn apart on the sharp reef.",
        "Sickness kept him home the third week.",
        "The wide road shimmered in the hot sun.",
        "The lazy cow lay in the cool grass.",
        "Lift the square stone over the fence.",
        "The rope will bind the seven books at once.",
        "Hop over the fence and plunge in.",
        "The friendly gang left the drug store.",
        "Mesh wire keeps chicks inside.",
        # Edge cases: Numbers and Contractions
        "I have 3 cats and 12 dogs.",
        "I don't know if he'll come.",
        "She's going to the store.",
        "We've seen enough.",
        "There are 100 ways to win."
    ]

    fixtures = []

    for text in test_sentences:
        # 1. Preprocess
        preprocessed = preprocessor(text)
        
        # 2. Phonemize
        phonemes_list = espeak_phonemizer.phonemize([preprocessed])
        raw_phonemes = phonemes_list[0]
        
        # 3. Tokenize (basic_english_tokenize)
        phoneme_tokens = basic_english_tokenize(raw_phonemes)
        phonemes_joined = ' '.join(phoneme_tokens)
        
        # 4. Clean (convert to IDs)
        tokens = text_cleaner(phonemes_joined)
        
        # 5. Add start/end tokens (as done in _prepare_inputs)
        # tokens.insert(0, 0)
        # tokens.append(10)
        # tokens.append(0)
        
        fixtures.append({
            "input": text,
            "preprocessed": preprocessed,
            "phonemes": raw_phonemes,
            "phonemes_joined": phonemes_joined,
            "tokens": tokens
        })

    # Save to JSON
    current_dir = os.path.dirname(os.path.abspath(__file__))
    output_path = os.path.join(current_dir, "python_outputs.json")
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(fixtures, f, indent=2, ensure_ascii=False)

    print(f"Fixtures generated successfully at {output_path}")

if __name__ == "__main__":
    generate_fixtures()

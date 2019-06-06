'''
This file codes and decodes plain text into a one time pad cipher
'''
import random

# Function definitions 

'''
The stringPrepper function reads a text file and returns a nested list structure where:

1. Each inner list represents a visual line of the text file:
2. All text is made lowercase
3. each character is replaced by a number

Ex text:

ABC
Another Line

Ex output:

[[97, 98, 99], [97, 110, 111, 116, 104, 101, 114, 32, 108, 105, 110, 101]]
'''
def stringPrepper(file_path): 
    message = open(file_path, mode="r")
    strings = message.readlines()
    message.close()

    for index, line in enumerate(strings): # Loops through text file and removes newline characters and turns all text to lowercase
        parsed_line = line.replace("\n","")
        strings[index] = parsed_line.lower()
    
    seperated_str = []
    for line in strings: # This takes each visual line from the text file and puts it into its own list and places it inside the main container 'seperated_str'
        chars = []
        for char in line:
            chars.append(char)
        seperated_str.append(chars)

    prepped = []
    for line in seperated_str: # Uses the ord() method to change every character in the text to a number
        line_container = []
        for char in line:
            letter_num = (ord(char))
            line_container.append(letter_num)
        prepped.append(line_container)

    return(prepped)

def keyGenerator(prepped_list): # This function chooses a pseudo-random number for every character in the text.
    key = []
    for line in prepped:
        line_vals = []
        for num in line:
            key_val = random.randrange(32,122)
            line_vals.append(key_val)
        key.append(line_vals)

    return(key)    
'''
The otpEncoder function encodes the plain text by:

1. Creates a zip object of the prepped list and the key. A zip object is (from the python docs):
    "Returns an iterator of tuples, where the i-th tuple contains the i-th element from each of the argument sequences or iterables."
2. Adds the character (which has been converted to a number) and the key, mod 122. The math is done mod 122 because of the number range that the ord() method can return
3. Turns that new number back into a character and puts it into the 'line_container" list
4. Takes the line container and turns it into one complete string, then puts that into the 'cipher_text' list and returns it
'''
def otpEncoder(prepped_list, key):
    cipher_text = []
    for line1, line2 in zip(prepped, key):
        line_container = []
        for char, key in zip(line1, line2):
            cipher_step1 = int(char) + int(key)
            cipher_step2 = cipher_step1 % 122
            print(cipher_step2)
            cipher_char = chr(cipher_step2)
            line_container.append(cipher_char)
        line_str = "".join(line_container)
        line_str = line_str + "\n"
        cipher_text.append(line_str)
    return(cipher_text)
    
'''
The otpDecoder function does the exact same thing as the code above except it subtracts the character and key values instead of adding them. This reverses the encoding and turns
the cipher back into plain text. 
'''

def otpDecoder(cipher_path, key):
    key_file = open(key, mode = "r")
    key_data = key_file.readlines()
    key_file.close()

    # Strip newlines out of the key text
    for index, line in enumerate(key_data):
        parsed_line = line.replace("\n", "")
        key_data[index] = parsed_line


    # Split into lists of characters????
    key = []
    for line in key_data:
        parsed_line = line.split()
        key.append(parsed_line)

    cipher_file = open(cipher_path, mode = "r")
    cipher_text = cipher_file.readlines()
    cipher_file.close()

    # Strip newlines out of the cipher text
    for index, line in enumerate(cipher_text):
        parsed_line = line.replace("\n", "")
        cipher_text[index] = parsed_line


    # Split the cipher text back into characters
    seperated_str = []
    for line in cipher_text:
        chars = []
        for char in line:
            chars.append(char)
        seperated_str.append(chars)

    # Turn the strings into ints
    prepped = []
    for line in seperated_str:
        line_container = []
        for char in line:
            letter_num = ord(char)
            line_container.append(letter_num)
        prepped.append(line_container)

    # Zip the cipher and the key and invert the encoding function (supposedly)
    decoded_text = []
    for (line1, line2) in zip(prepped, key):
        line_container = []
        for char, key in zip(line1, line2):
            cipher_step1 = int(char) - int(key)
            cipher_step2 = cipher_step1 % 122
            cipher_char = chr(cipher_step2)
            line_container.append(cipher_char)
        decoded_text.append(line_container)

    decoded_str = []

    for line in decoded_text:
        line_str =  "".join(line)
        line_str = line_str + "\n"
        decoded_str.append(line_str)

    return decoded_str

# End function definitions

file_path = input("Welcome, please provide a file path: ")
mode = input("Would you like to encode (E) or decode (D) the file: ")

if mode == "E":
    prepped = stringPrepper(file_path)
    key = keyGenerator(prepped)

    otp_id = random.randint(0,9999) # Chooses a 4 digit ID that can be used to match cipher text and its key.
    key_file = "OTK_" + str(otp_id)
    output_k = open(key_file, mode="w+")
    
    for line in key: # Writes the key to the new text file created above
        key_values = []
        base = ""
        string_container = []
        for num in line:
            key_values.append(str(num) + " ")
        key_str = base.join(key_values)
        key_str = key_str + "\n"
        string_container.append(key_str)
        for line in string_container:
            output_k.writelines(line)
    output_k.close()

    # Writes the cipher text to a file
    cipher_text = otpEncoder(prepped, key) 
    cipher_file = "OTP_" + str(otp_id)
    output_c = open(cipher_file, mode="w+")
    for line in cipher_text:
        output_c.writelines(line)
    output_c.close()

# Decodes a cipher given both the cipher and its key, then writes the decoded text to a file
elif mode == "D": 
    cipher_path = input("File path to cipher: ")
    key_path = input("File path to key: ")
    decoded_text = otpDecoder(cipher_path, key_path)

    output_filename = "Decoded_" + str(random.randint(0,9999))
    output_decoded = open(output_filename, mode="w+")
    for line in decoded_text:
        output_decoded.writelines(line)
    output_decoded.close()

elif mode != "E" or "D":
    pass

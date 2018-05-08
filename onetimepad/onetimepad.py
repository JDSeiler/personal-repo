'''
This file codes and decodes plain text into a one time pad cipher
'''
import random
#function definitions... 

def stringPrepper(file_path):
    file_path = file_path
    message = open(file_path, mode="r")
    strings = message.readlines()
    message.close()

    for line in strings:
        loc = strings.index(line)
        parsed_line = line.replace("\n","")
        strings.pop(loc)
        strings.insert(loc, parsed_line.lower())
    
    seperated_str = []
    for line in strings:
        chars = []
        for char in line:
            chars.append(char)
        seperated_str.append(chars)

    prepped = []
    for line in seperated_str:
        line_container = []
        for char in line:
            letter_num = (ord(char))
            line_container.append(letter_num)
        prepped.append(line_container)

    return(prepped)

def keyGenerator(prepped_list):
    key = []
    for line in prepped:
        line_vals = []
        for num in line:
            key_val = random.randrange(32,122)
            line_vals.append(key_val)
        key.append(line_vals)

    return(key)    

def otpEncoder(prepped_list, key):
    cipher_text = []
    for line1, line2 in zip(prepped, key):
        line_container = []
        for char, key in zip(line1, line2):
            cipher_step1 = int(char) + int(key)
            cipher_step2 = cipher_step1 % 122
            cipher_char = chr((cipher_step2))
            line_container.append(cipher_char)

        base = ""
        line_str = base.join(line_container)
        line_str = line_str + "\n"
        cipher_text.append(line_str)

    return(cipher_text)

def otpDecoder(cipher_file, key):

    cipher_path = cipher_file
    key_path = key

    key_file = open(key_path, mode = "r")
    key_data = key_file.readlines()
    key_file.close()

    for line in key_data:
        loc = key_data.index(line)
        parsed_line = line.replace("\n", "")
        key_data.pop(loc)
        key_data.insert(loc, parsed_line)

    key = []
    for line in key_data:
        parsed_line = line.split()
        key.append(parsed_line)

    cipher_file = open(cipher_path, mode = "r")
    cipher_text = cipher_file.readlines()
    cipher_file.close()

    for line in cipher_text:
        loc = cipher_text.index(line)
        parsed_line = line.replace("\n", "")
        cipher_text.pop(loc)
        cipher_text.insert(loc, parsed_line)

    seperated_str = []
    for line in cipher_text:
        chars = []
        for char in line:
            chars.append(char)
        seperated_str.append(chars)
    prepped = []
    for line in seperated_str:
        line_container = []
        for char in line:
            letter_num = (ord(char))
            line_container.append(letter_num)
        prepped.append(line_container)

    decoded_text = []

    for (line1, line2) in zip(prepped, key):
        line_container = []
        for char, key in zip(line1, line2):
            cipher_step1 = int(char) - int(key)
            cipher_step2 = cipher_step1 % 122
            cipher_char = chr((cipher_step2))
            line_container.append(cipher_char)
        decoded_text.append(line_container)

    for line in decoded_text:
        for char in line:
            if char == "\x00":
                loc = line.index(char)
                line.pop(loc)
                line.insert(loc, "z")
            else:
                pass

    decoded_str = []

    for line in decoded_text:
        base = ""
        line_str =  base.join(line)
        line_str = line_str + "\n"
        decoded_str.append(line_str)

    return decoded_str

file_path = input("Welcome, please provide a file path: ")
mode = input("Would you like to encode (E) or decode (D) the file: ")

if mode == "E":
    prepped = stringPrepper(file_path)
    key = keyGenerator(prepped)

    otp_id = random.randint(0,9999)
    key_file = "OTK_" + str(otp_id)
    output_k = open(key_file, mode="w+")
    
    for line in key:
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

    cipher_text = otpEncoder(prepped, key)
    cipher_file = "OTP_" + str(otp_id)
    output_c = open(cipher_file, mode="w+")
    for line in cipher_text:
        output_c.writelines(line)
    output_c.close()

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
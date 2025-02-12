#!/usr/bin/python3

import sys
import os
import subprocess

total_points = 0


def part_one_test(test_name,command,expected):
    global total_points
    test_one = subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)

    if int(test_one.stdout.decode().strip()) == expected:
        print(f'test case {test_name}, input {command[1]} - passed!')
        total_points = total_points + 1
    else:
        print(f'test case {test_name}, input {command[1]} - failed! expected: {expected}, actual: {test_one.stdout.decode().strip()}')
    
def grade_part_one():
    print('===part one - grading===')
    os.chdir('first')
    make_command = subprocess.run(['make','clean'],stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if make_command.returncode != 0:
        print('make clean - FAILED!')
        print(make_command.stdout.strip())
        return -1
    subprocess.run(['rm','-f','first'])

    make_command = subprocess.run(['make','first'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    if make_command.returncode != 0:
        print('make first - FAILED!')
        print(make_command.stdout.strip())
        return -1
    
    if not os.path.isfile("first"):
        print('no first executable - FAILED!')
        return -1
    # tests
    part_one_test('one',['./first','6'],3)
    part_one_test('two',['./first','28'],7)
    part_one_test('three',['./first','8128'],127)
    part_one_test('four',['./first','120'],-1)
    part_one_test('five',['./first','33550336'],8191)

    # print(f'total tests passed: {total_points}/5. remember, there maybe hidden test cases. perform additional testing')

    os.chdir('..')

def part_two_test(test_name, commands, expected_file):
    global total_points
    
    # Run the subprocess and capture its output
    test_case = subprocess.run(commands, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print('========================')
    output = test_case.stdout.decode('utf-8')

    try:

        with open(expected_file, "r") as file:
            expected_output = file.read()

        if output == expected_output:
            print(f"test case {test_name}: passed!")
            total_points = total_points + 1
        else:
            print(f"test case {test_name}: failed!")
            print("=====actual=====")
            print(test_case.stdout.decode())
            print("====expected====")
            print(expected_output)


    except FileNotFoundError:
        print(f"Error: Expected file '{expected_file}' not found.")
        
    except Exception as e:
        print(f"An error occurred: {e}")


def grade_part_two():
    print('===part two - grading===')
    os.chdir('second')

    make_command = subprocess.run(['make','clean'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)

    if make_command.returncode != 0:
        print('make clean - FAILED!')
        print(make_command.stdout.strip())
        return -1
    subprocess.run(['rm','-f','second'])

    make_command = subprocess.run(['make','second'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    
    if make_command.returncode != 0:
        print('make second - FAILED!')
        print(make_command.stdout.strip())
        return -1
    
    if not os.path.isfile("second"):
        print("no second executable!")
        return -1
    tests = ['one','two','three','four','five','six']
    for i in range(1, 6):  # Loop from 1 to 5
        test_namer = f"{tests[i - 1]}"
        commands = ["./second", f"../../testcases/second/test{i}.txt"]
        expected_file = f"../../testcases/second/test{i}.out"
        part_two_test(test_namer, commands, expected_file)
    
    # print(f'total tests passed: {total_points}/5. remember, there maybe hidden test cases. perform additional testing')

    # grading
    os.chdir('..')


def part_three_to_five_test(test_name, commands, expected_file):
    global total_points
    
    # Run the subprocess and capture its output
    test_case = subprocess.run(commands, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    # print('========================')
    output = test_case.stdout.decode('utf-8')

    try:

        with open(expected_file, "r") as file:
            expected_output = file.read()

        if output == expected_output:
            print(f"test case {test_name}: passed!")
            total_points = total_points + 1
        else:
            print(f"test case {test_name}: failed!")
            print("=====actual=====")
            print(test_case.stdout.decode())
            print("====expected====")
            print(expected_output)


    except FileNotFoundError:
        print(f"Error: Expected file '{expected_file}' not found.")
        
    except Exception as e:
        print(f"An error occurred: {e}")


def grade_part_three():
    print('===part three- grading===')
    os.chdir('third')

    make_command = subprocess.run(['make','clean'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)

    if make_command.returncode != 0:
        print('make clean - FAILED!')
        print(make_command.stdout.strip())
        return -1
    subprocess.run(['rm','-f','third'])

    make_command = subprocess.run(['make','third'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    
    if make_command.returncode != 0:
        print('make third - FAILED!')
        print(make_command.stdout.strip())
        return -1
    
    if not os.path.isfile("third"):
        print("no third executable!")
        return -1
    tests = ['one','two','three','four','five','six']
    for i in range(1, 6):  # Loop from 1 to 5
        test_namer = f"{tests[i - 1]}"
        commands = ["./third", f"../../testcases/third/test{i}.txt"]
        expected_file = f"../../testcases/third/test{i}.out"
        part_three_to_five_test(test_namer, commands, expected_file)
    
    # print(f'total tests passed: {total_points}/5. remember, there maybe hidden test cases. perform additional testing')

    # grading
    os.chdir('..')


def grade_part_four():
    print('===part four - grading===')
    os.chdir('fourth')

    make_command = subprocess.run(['make','clean'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)

    if make_command.returncode != 0:
        print('make clean - FAILED!')
        print(make_command.stdout.strip())
        return -1
    subprocess.run(['rm','-f','fourth'])

    make_command = subprocess.run(['make','fourth'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    
    if make_command.returncode != 0:
        print('make fourth - FAILED!')
        print(make_command.stdout.strip())
        return -1
    
    if not os.path.isfile("fourth"):
        print("no fourth executable!")
        return -1
    tests = ['one','two','three','four','five','six']
    for i in range(1, 6):  # Loop from 1 to 5
        test_namer = f"{tests[i - 1]}"
        commands = ["./fourth", f"../../testcases/fourth/test{i}.txt"]
        expected_file = f"../../testcases/fourth/test{i}.out"
        part_three_to_five_test(test_namer, commands, expected_file)
    
    # print(f'total tests passed: {total_points}/5. remember, there maybe hidden test cases. perform additional testing')

    # grading
    os.chdir('..')


def grade_part_five():
    print('===part five - grading===')
    os.chdir('fifth')

    make_command = subprocess.run(['make','clean'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)

    if make_command.returncode != 0:
        print('make clean - FAILED!')
        print(make_command.stdout.strip())
        return -1
    subprocess.run(['rm','-f','fifth'])

    make_command = subprocess.run(['make','fifth'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    
    if make_command.returncode != 0:
        print('make fifth - FAILED!')
        print(make_command.stdout.strip())
        return -1
    
    if not os.path.isfile("fifth"):
        print("no fourth executable!")
        return -1
    tests = ['one','two','three','four','five','six']
    for i in range(1, 6):  # Loop from 1 to 5
        test_namer = f"{tests[i - 1]}"
        commands = ["./fifth", f"../../testcases/fifth/test{i}.txt"]
        expected_file = f"../../testcases/fifth/test{i}.out"
        part_three_to_five_test(test_namer, commands, expected_file)

    # grading
    os.chdir('..')

def remove_directory(directory):
    # Walk the directory tree in reverse order
    for root, dirs, files in os.walk(directory, topdown=False):
        # Remove all files
        for file in files:
            file_path = os.path.join(root, file)
            os.remove(file_path)
        for dir in dirs:
            dir_path = os.path.join(root, dir)
            os.rmdir(dir_path)
    os.rmdir(directory)

def grade_assignment(part_number,individual):
    os.chdir('./pa1')
    if part_number == 1:
        grade_part_one()
    elif part_number == 2:
        grade_part_two()
    elif part_number == 3:
        grade_part_three()
    elif part_number == 4:
        grade_part_four()
    elif part_number == 5:
        grade_part_five()
    if individual == True:
        print(f'total tests passed: {total_points}/5. remember, there maybe hidden test cases. perform additional testing')

    os.chdir('..')


def main():
    global question
    question = -1  # Default value if --part is not provided

    # Parse command-line arguments
    for i, arg in enumerate(sys.argv):
        if arg == "--part" and i + 1 < len(sys.argv):
            try:
                question = int(sys.argv[i + 1])  # Convert the next argument to an integer
            except ValueError:
                if sys.argv[i + 1] == "all":
                    for parts in range(1,6):
                        grade_assignment(parts,False)
                    print(f'total tests passed: {total_points}/25. Remember, there maybe hidden test cases. Perform additional testing')
                    exit(0)
                else:
                    print("Error: The value for --part must be an integer.")
                    sys.exit(1)
        elif arg == "pa1.tar":
            if os.path.isdir('pa1'):
                os.rename('pa1','pa1-backup')
                print('your pa1 directory has been renamed to pa1-backup')
            if not os.path.isfile('pa1.tar'):
                print('pa1.tar does not exist, exiting...')
                exit(1)
            extract_tar = subprocess.run(['tar','-xvf','pa1.tar'])
            if extract_tar.returncode != 0:
                print('Error extracting pa1.tar, exiting...')
                exit(1)
            for parts in range(1,6):
                grade_assignment(parts,False)
            print(f'total tests passed: {total_points}/25. Remember, there maybe hidden test cases. Perform additional testing')
            remove_directory('pa1')
            if os.path.isdir('pa1-backup'):
                os.rename('pa1-backup','pa1')
            exit(0)

    if question == -1:
        print("No part specified. Use '--part <number>' to set the part number.")
    else:
        grade_assignment(question,True)
        

if __name__ == "__main__":
    main()

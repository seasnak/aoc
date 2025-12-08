#include <cmath>
#include <cstdlib>
#include <iostream>
#include <string>
#include <fstream>

struct RotationAction {
    int finish {0};
    int clicks {0};

};

int rotate_dial(int start, std::string input)
{
    int finish {start};
    int num_rotations = std::stoi(input.substr(1, input.length()-1));

    if(input[0] == 'L')
    {
        finish = (finish - num_rotations) % 100;
        if (finish < 0) finish = 100 + finish;
    }
    else if (input[0] == 'R')
    {
        finish = (finish + num_rotations) % 100;
    }

    return finish; 
}

RotationAction rotate_dial_count_clicks(int start, std::string input)
{
    int num_rotations = std::stoi(input.substr(1, input.length()-1));

    RotationAction output;
    output.finish = start;
    output.clicks = 0;
    
    if (input[0] == 'L')
    {
        output.finish = (output.finish - num_rotations) % 100;
        output.clicks = num_rotations / 100;
        
        if (output.finish < 0) {
            // output.finish = 100 + output.finish;
            output.finish += 100;
            if (start != 0) output.clicks++;
        }
        else if (output.finish == 0) {
            output.clicks++;
        }

    }
    else if (input[0] == 'R')
    {
        // output.finish = (output.finish + num_rotations);
        output.finish += num_rotations;
        output.clicks += output.finish / 100;
        output.finish = output.finish % 100;

        // if (start == 0) output.clicks --;
    }
    
    return output; 
}

int get_password(int start, std::string inputs_filename, int part)
{
    std::ifstream inputs_fs;
    std::string line;
    int finish {start};
    int start_tmp {start};
    int password {0};
    RotationAction output;

    inputs_fs.open(inputs_filename);
    if (inputs_fs.is_open())
    {
        while (getline(inputs_fs, line))
        {
            if (line.empty()) continue;
            // std::cout << finish << " -> " << line;
            start_tmp = finish;
            output = rotate_dial_count_clicks(finish, line);
            finish = output.finish;
            // std::cout << " = " << finish;

            if (finish == 0) 
            {
                // output.clicks++;
            }

            if (line[0] == 'R' && output.clicks == 1)
            {
                std::cout << start_tmp << " -> " << line << " = " << finish << " --- " << output.clicks << std::endl;
            }
            password += output.clicks;

        }

        inputs_fs.close();
    }
    else std::cout << "Unable to open file" << std::endl;
    
    return password;
}

int main()
{
    std::string input_fname {"../../data/y25/d01.txt"};
    int start = 50;
    std::cout << get_password(start, input_fname, 2) << std::endl;
}

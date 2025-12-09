#include <cmath>
#include <iostream>
#include <stdlib.h>

#include <fstream>
#include <sstream>
#include <string>

struct IDRange
{
    std::string start {""};
    std::string end {""};
};

std::string readFileToString(const std::string& filename) {
    std::ifstream inputFileStream(filename);
    std::stringstream bufferStream;
    bufferStream << inputFileStream.rdbuf();
    return bufferStream.str();
}

unsigned long getInvalidIDTotalFromRange(const IDRange idr)
{
    unsigned long total {0};

    // case: can't form repeated string from an odd number of characters
    if (idr.start.length() % 2 != 0 && idr.start.length() == idr.end.length()) return 0;
    if (idr.end.length() == 1) return 0;

    std::cout << "start: " << idr.start << " end: " << idr.end << std::endl;

    unsigned long start_i = std::stoul(idr.start);
    unsigned long end_i = std::stoul(idr.end);

    std::string start_half {""};
    if ( idr.start.length() == 1 ) start_half = "0";
    else if( idr.start.length() %2 == 0) start_half = idr.start.substr(0, idr.start.length() / 2);
    else start_half = idr.start.substr(0, (idr.start.length() / 2));

    std::string end_half {""};
    if (idr.end.length() % 2 == 0) end_half = idr.end.substr(0, idr.end.length() / 2);
    else end_half = idr.end.substr(0, (idr.end.length() / 2) + 1);

    unsigned long start_half_i { std::stoul(start_half) };
    unsigned long end_half_i { std::stoul(end_half) };

    // std::cout << "start half: " << start_half << " end half: " << end_half << std::endl;

    for(unsigned long i {start_half_i}; i <= end_half_i; i++)
    {
        std::string current_id = std::to_string(i) + std::to_string(i);
        // std::cout << "current id: " << current_id << std::endl;
        unsigned long current_id_i = std::stoul(current_id);

        if (current_id_i >= start_i && current_id_i <= end_i)
        {
            // std::cout << "found id: " << current_id_i << std::endl;
            total += current_id_i;
        }
    }

    // std::cout << "total: " << total <<  std::endl;
    return total;
}

std::string createLoopedString(std::string input, int num_repeats)
{
    std::string output {""};

    for(int i = 0; i < num_repeats; i++)
    {
        output += input;
    }

    return output;
}

unsigned long getNInvalidIDSubtotal(const IDRange idr, int split_size)
{
    unsigned long total {0};
    
    unsigned long start_num = std::stoul(idr.start);
    unsigned long end_num = std::stoul(idr.end);
    if (idr.start.length() % split_size != 0 && idr.end.length() == idr.start.length())
    {
        return 0;
    }

    std::string start_substr {};
    if (idr.start.length() % split_size == 0) 
    {
        start_substr = idr.start.substr(0, split_size);
    }
    else 
    {
        start_substr = idr.start.substr(0, split_size + 1);
    }
    
    std::string end_substr {};
    if (idr.end.length() > idr.start.length())
    {
        end_substr = idr.end.substr(0, split_size + (idr.end.length() - idr.start.length()));
    }
    else
    {
        if  (idr.end.length() % 2 == 0)
        {
            end_substr = idr.end.substr(0, split_size);
        }
        else
        {
            end_substr = idr.end.substr(0, split_size + 1);
        }
    }

    std::cout << "start substr: " << start_substr << " end substr: " << end_substr << std::endl;

    unsigned long start_substr_num = std::stoul(start_substr);
    unsigned long end_substr_num = std::stoul(end_substr);

    int num_loops = idr.start.length() / split_size;

    for (unsigned long subid = start_substr_num; subid <= end_substr_num; subid++)
    {
        // std::cout << "current subid: " << subid << ", ";
        std::string current_id { createLoopedString( std::to_string(subid), num_loops ) };
        // std::cout << "checking id: " << current_id << std::endl;
        unsigned long current_id_num = std::stoul(current_id);

        if (current_id_num >= start_num && current_id_num <= end_num)
        {
            std::cout << "found illegal id: " << current_id << std::endl;
            total += current_id_num;
        }
        
        if (idr.start.length() == idr.end.length()) continue;

        std::string current_id2 { createLoopedString(std::to_string(subid), idr.start.length() / std::to_string(subid).length()) };
        std::cout << "checking id: " << current_id2 << std::endl;
        unsigned long current_id_num2 = std::stoul(current_id2);

        if (current_id_num2 >= start_num && current_id_num2 <= end_num)
        {
            total += current_id_num2;
        }
    }

    return total;
}

unsigned long getNInvalidIDTotalFromRange(const IDRange idr)
{
    unsigned long total {0};

    std::cout << idr.start << "--" << idr.end << std::endl;

    int start_split_size_cap = (idr.start.length() / 2);
    int end_split_size_cap = (idr.end.length() / 2);

    bool start_size_is_even { idr.start.length() % 2 == 0 };
    bool end_size_is_even { idr.end.length() % 2 == 0 };

    for (int split_size = 1; split_size <= end_split_size_cap; split_size++)
    {
        total += getNInvalidIDSubtotal(idr, split_size);
    }
    

    return total;
}

unsigned long getInvalidIDTotal(const std::string& input_fname, int part)
{
    std::string content = readFileToString(input_fname);

    std::stringstream ss(content);
    std::string token;
    unsigned long total = 0;

    IDRange id_range;
    
    while (std::getline(ss, token, ',')) 
    {
        if (!token.empty())
        {
            // std::cout << "token: " << token << std::endl;
            std::stringstream id_ss(token);

            std::getline(id_ss, id_range.start, '-');
            std::getline(id_ss, id_range.end);
            // std::cout << "start: " << id_range.start << " end: " << id_range.end << std::endl;

            if (part == 1) total += getInvalidIDTotalFromRange(id_range);
            else if (part == 2) total += getNInvalidIDTotalFromRange(id_range);
        }
        // std::cout << "running total: " << total << std::endl;
    }

    return total;
}

int main() {

    std::string input_fname {"../../data/y25/d02-ex.txt"};

    // PART ONE
    // std::cout << getInvalidIDTotal(input_fname, 1) << std::endl;

    // PART TWO
    std::cout << getInvalidIDTotal(input_fname, 2) << std::endl;

    return EXIT_SUCCESS;
}

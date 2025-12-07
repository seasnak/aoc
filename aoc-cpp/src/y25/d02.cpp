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
    if (idr.start.length() %2 != 0 && idr.end.length() == idr.start.length()) return 0;
    else if (idr.end.length() == 1) return 0;

    std::cout << "start: " << idr.start << " end: " << idr.end << std::endl;

    unsigned long start_i = std::stoul(idr.start);
    unsigned long end_i = std::stoul(idr.end);

    std::string start_half {"0"};
    if( idr.start.length() != 1)
    {
        start_half = idr.start.substr(0, idr.start.length() / 2);
    }
    std::string end_half { idr.end.substr(0, idr.end.length() / 2) };

    unsigned long start_half_i { std::stoul(start_half) };
    unsigned long end_half_i { std::stoul(end_half) };

    for(unsigned long i {start_half_i}; i <= end_half_i; i++)
    {
        std::string current_id = std::to_string(i) + std::to_string(i);
        std::cout << "current id: " << current_id << std::endl;
        unsigned long current_id_i = std::stoul(current_id);

        if (current_id_i > start_i && current_id_i < end_i)
        {
            total += current_id_i;
        }
    }
    
    return total;    
}

int getInvalidIDTotal(const std::string& input_fname)
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

            total += getInvalidIDTotalFromRange(id_range);
        }
        

    }

    return total; 
}

int main() {

    std::string input_fname {"../../data/y25/d02.txt"};

    // PART ONE
    std::cout << getInvalidIDTotal(input_fname) << std::endl;

    // PART TWO

    return EXIT_SUCCESS;
}

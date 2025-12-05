#include <iostream>
#include <stdlib.h>

#include <fstream>
#include <sstream>

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

int getInvalidIDTotalFromRange(const IDRange idr)
{
    int total {0};

    // case: can't form repeated string from an odd number of characters
    if (idr.start.length() %2 != 0 && idr.end.length() == idr.start.length()) return 0;
    
    return total;    
}

int getInvalidIDTotal(const std::string& input_fname)
{
    std::string content = readFileToString(input_fname);

    std::stringstream ss(content);
    std::string token;
    int total = 0;

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
        }
        
        total += getInvalidIDTotalFromRange(id_range);

    }

    return total; 
}

int main() {

    std::string input_fname {"../../data/y25/d02.txt"};

    getInvalidIDTotal(input_fname);

    return EXIT_SUCCESS;
}

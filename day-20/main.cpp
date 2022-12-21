#include <iostream>
#include <memory>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

struct Node {
    int64_t value;
    Node* previous;
    Node* next;
};

// Mix the list to decrypt
static void decrypt(std::vector<Node>& nodes, unsigned times) {
    while (times--) {
        for (unsigned i = 0; i < nodes.size(); i++) {
            auto* value = &nodes[i];
            auto* head = value;

            int64_t rotate = value->value % static_cast<int64_t>(nodes.size() - 1);                        
            if (rotate == 0) { continue; }

            // Remove current node from linked list
            head->previous->next = head->next;
            head->next->previous = head->previous;
            head = head->previous;

            auto count = std::llabs(rotate);
            while (count--) {
                head = (rotate < 0) ? head->previous : head->next;
            }
            value->previous = head;
            value->next = head->next;
            head->next->previous = value;
            head->next = value;
        }
    }
}


int main(int argc, char* argv[]) {    
    std::fstream input("input.txt", std::ios_base::in);

    // Read lines into a vector of nodes: this is used to maintain original order
    std::vector<Node> list;    
    for( std::string line; std::getline( input, line ); ) {
        auto value = std::stoi(line);
        list.push_back(Node{value, nullptr, nullptr});                
    }

    // Assign previous and next pointers
    list[0].previous = &list[list.size() - 1];
    list[list.size() - 1].next = &list[0];
    for (unsigned i = 0; i < list.size(); i++) {
        if (i > 0) {
            list[i].previous = &list[i - 1];
        }
        if (i < list.size() - 1) {
            list[i].next = &list[i+1];
        }
    }

    // Part one
    decrypt(list, 1);
    auto it = std::find_if(list.begin(), list.end(), [](auto n){ return n.value == 0; });
    auto* node = &list[it - list.begin()];
    std::vector<int> values(list.size());
    for (unsigned i = 0; i < values.size(); i++) {
        values[i] = node->value;
        node = node->next;
    }    
    
    auto sum = values[1000 % list.size()] + values[2000 % list.size()] + values[3000 % list.size()];
    std::cout << "Part one: " << sum << "\n";

    // Part two: Note: Comment out part one to run because I'm too lazy to clone the list
    for (auto& node : list) {
        node.value *= int64_t{811589153};
    }
    decrypt(list, 10);
    auto it = std::find_if(list.begin(), list.end(), [](auto n){ return n.value == 0; });
    auto* node = &list[it - list.begin()];
    std::vector<int64_t> values(list.size());
    for (unsigned i = 0; i < values.size(); i++) {
        values[i] = node->value;
        node = node->next;
    }    
    
    auto sum = values[1000 % list.size()] + values[2000 % list.size()] + values[3000 % list.size()];
    std::cout << "Part two: " << sum << "\n";

    return 0;
}


